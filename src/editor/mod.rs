//! # Editor Module
//!
//! This module contains the core Editor implementation for the Hecto text editor.
//! The Editor serves as the main application controller, orchestrating the interaction
//! between different UI components and handling user input events.
//!
//! ## Architecture
//!
//! The Editor follows a component-based architecture with the following key components:
//! - **View**: Handles text display and editing operations
//! - **StatusBar**: Shows document information and cursor position
//! - **MessageBar**: Displays informational messages to the user
//! - **CommandBar**: Handles user input during prompts (save, search)
//! - **Terminal**: Manages low-level terminal operations
//!
//! ## Event Handling
//!
//! The editor processes events in a main loop, converting terminal events into
//! high-level commands and dispatching them to appropriate handlers based on
//! the current application state (normal editing, search mode, save prompt).

use crate::prelude::*;
use std::{
    env,
    io::Error,
    panic::{set_hook, take_hook},
};

use crossterm::event::{Event, KeyEvent, KeyEventKind, read};

// Module declarations
mod annotatedstring;
mod command;
mod documentstatus;
mod line;
mod terminal;
mod uicomponents;

use self::{
    annotatedstring::{AnnotatedString, AnnotationType},
    command::{
        Command::{self, Edit, Move, System},
        Edit::InsertNewline,
        Move::{Down, Left, Right, Up},
        System::{Dismiss, Quit, Resize, Save, Search},
    },
    documentstatus::DocumentStatus,
    line::Line,
    terminal::Terminal,
    uicomponents::{CommandBar, MessageBar, StatusBar, UIComponent, View},
};

/// Number of consecutive Ctrl+Q presses required to quit with unsaved changes
const QUIT_TIMES: u8 = 3;

/// Represents the current prompt state of the editor
///
/// The editor can be in different prompt modes where user input is interpreted
/// differently than normal text editing.
#[derive(Eq, PartialEq, Default)]
enum PromptType {
    /// Search mode - user is entering a search query
    Search,
    /// Save mode - user is entering a filename to save
    Save,
    /// Normal editing mode - no active prompt
    #[default]
    None,
}

impl PromptType {
    /// Returns true if the editor is not in any prompt mode
    fn is_none(&self) -> bool {
        *self == Self::None
    }
}

/// Main Editor structure containing all application state
///
/// The Editor serves as the central coordinator for the text editor application,
/// managing the interaction between various UI components and handling the main
/// event loop.
#[derive(Default)]
pub struct Editor {
    /// Flag indicating whether the application should exit
    should_quit: bool,
    /// The main text editing view
    view: View,
    /// Status bar displaying document information
    status_bar: StatusBar,
    /// Message bar for displaying informational messages
    message_bar: MessageBar,
    /// Command bar for user input during prompts
    command_bar: CommandBar,
    /// Current prompt mode
    prompt_type: PromptType,
    /// Current terminal dimensions
    terminal_size: Size,
    /// Window title
    title: String,
    /// Counter for consecutive quit attempts with unsaved changes
    quit_times: u8,
}
impl Editor {
    /// Creates a new Editor instance and initializes the terminal
    ///
    /// This method performs the following initialization steps:
    /// 1. Sets up a panic hook to ensure proper terminal cleanup on panic
    /// 2. Initializes the terminal (raw mode, alternate screen, etc.)
    /// 3. Sets up the initial UI components and their sizes
    /// 4. Loads a file if specified as a command-line argument
    /// 5. Displays the initial help message
    ///
    /// # Returns
    ///
    /// Returns a Result containing the initialized Editor on success, or an Error
    /// if terminal initialization fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - Terminal initialization fails (e.g., unable to enter raw mode)
    /// - Terminal size cannot be determined
    pub fn new() -> Result<Self, Error> {
        // Set up panic hook to ensure terminal is properly restored on panic
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));

        // Initialize terminal and editor state
        Terminal::initialize()?;
        let mut editor = Self::default();
        let size = Terminal::size().unwrap_or_default();
        editor.handle_resize_command(size);
        editor.update_message("HELP: Ctrl-F = find | Ctrl-S = save | Ctrl-Q = quit");

        // Load file if specified as command-line argument
        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            debug_assert!(!file_name.is_empty());
            if editor.view.load(file_name).is_err() {
                editor.update_message(&format!("ERR:Could not open file: {file_name}"));
            }
        }

        editor.refresh_status();
        Ok(editor)
    }

    /// Runs the main editor event loop
    ///
    /// This method implements the core event loop of the editor:
    /// 1. Refreshes the screen display
    /// 2. Checks if the application should quit
    /// 3. Reads and processes terminal events
    /// 4. Updates the status display
    /// 5. Repeats until `should_quit` is true
    ///
    /// The loop handles all user input and terminal events, converting them
    /// into appropriate commands and updating the UI accordingly.
    pub fn run(&mut self) {
        loop {
            self.refresh_screen();
            if self.should_quit {
                break;
            }
            match read() {
                Ok(event) => self.evaluate_event(event),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event: {err:?}");
                    }
                    #[cfg(not(debug_assertions))]
                    {
                        let _ = err;
                    }
                },
            }
            self.refresh_status();
        }
    }
    /// Refreshes the entire screen display
    ///
    /// This method orchestrates the rendering of all UI components in the correct order:
    /// 1. Hides the cursor during rendering
    /// 2. Renders the appropriate bottom bar (command bar if in prompt mode, message bar otherwise)
    /// 3. Renders the status bar if there's enough vertical space
    /// 4. Renders the main text view if there's enough vertical space
    /// 5. Positions the cursor appropriately (in command bar if in prompt mode, or in text view)
    /// 6. Shows the cursor and executes all queued terminal commands
    ///
    /// The method handles terminal size constraints gracefully, only rendering components
    /// that fit within the available space.
    fn refresh_screen(&mut self) {
        let Size { height, width } = self.terminal_size;

        // Don't render if terminal is too small
        if height == 0 || width == 0 {
            return;
        }

        let bottom_bar_row = height.saturating_sub(1);
        let _ = Terminal::hide_caret();

        // Render bottom bar (command bar in prompt mode, message bar otherwise)
        if self.in_prompt() {
            self.command_bar.render(bottom_bar_row);
        } else {
            self.message_bar.render(bottom_bar_row);
        }

        // Render status bar if there's room (need at least 2 rows)
        if height > 1 {
            self.status_bar.render(height.saturating_sub(2));
        }

        // Render main text view if there's room (need at least 3 rows)
        if height > 2 {
            self.view.render(0);
        }

        // Position cursor appropriately
        let new_caret_pos = if self.in_prompt() {
            Position {
                col: self.command_bar.caret_position_col(),
                row: bottom_bar_row,
            }
        } else {
            self.view.caret_position()
        };

        debug_assert!(new_caret_pos.col <= self.terminal_size.width);
        debug_assert!(new_caret_pos.row <= self.terminal_size.height);

        let _ = Terminal::move_caret_to(new_caret_pos);
        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }

    /// Updates the status bar and window title based on the current document state
    ///
    /// This method retrieves the current document status from the view and updates
    /// both the status bar display and the terminal window title. The title format
    /// is "{filename} - {application_name}".
    pub fn refresh_status(&mut self) {
        let status = self.view.get_status();
        let title = format!("{} - {NAME}", status.file_name);
        self.status_bar.update_status(status);
        if title != self.title && matches!(Terminal::set_title(&title), Ok(())) {
            self.title = title;
        }
    }
    /// Evaluates and processes terminal events
    ///
    /// This method filters incoming terminal events to determine which ones should be processed.
    /// Only key press events and resize events are handled; other events are ignored.
    ///
    /// # Arguments
    ///
    /// * `event` - The terminal event to evaluate
    fn evaluate_event(&mut self, event: Event) {
        let should_process = match &event {
            Event::Key(KeyEvent { kind, .. }) => kind == &KeyEventKind::Press,
            Event::Resize(_, _) => true,
            _ => false,
        };
        if should_process {
            if let Ok(command) = Command::try_from(event) {
                self.process_command(command);
            }
        }
    }

    /// Processes a command based on the current editor state
    ///
    /// This method serves as the main command dispatcher, routing commands to appropriate
    /// handlers based on the current prompt type. Resize commands are handled immediately
    /// regardless of the current state.
    ///
    /// # Arguments
    ///
    /// * `command` - The command to process
    fn process_command(&mut self, command: Command) {
        // Handle resize commands immediately
        if let System(Resize(size)) = command {
            self.handle_resize_command(size);
            return;
        }

        // Route command based on current prompt state
        match self.prompt_type {
            PromptType::Save => self.process_command_during_save(command),
            PromptType::Search => self.process_command_during_search(command),
            PromptType::None => self.process_command_no_prompt(command),
        }
    }

    /// Processes commands when no prompt is active (normal editing mode)
    ///
    /// In normal mode, the editor handles:
    /// - Quit commands (with unsaved changes protection)
    /// - Search activation
    /// - Save operations
    /// - Text editing commands
    /// - Cursor movement commands
    ///
    /// # Arguments
    ///
    /// * `command` - The command to process
    fn process_command_no_prompt(&mut self, command: Command) {
        // Handle quit command specially (may require confirmation)
        if matches!(command, System(Quit)) {
            self.handle_quit_command();
            return;
        }

        // Reset quit counter on any other command
        self.reset_quit_times();

        match command {
            System(Search) => self.set_prompt(PromptType::Search),
            System(Save) => self.handle_save_command(),
            Edit(edit_command) => self.view.handle_edit_command(edit_command),
            Move(move_command) => self.view.handle_move_command(move_command),
            System(_) => {}, // Other system commands are ignored in normal mode
        }
    }
    /// Handles terminal resize events
    ///
    /// This method updates the terminal size and resizes all UI components accordingly.
    /// The view gets the remaining space after reserving space for the status and message bars.
    ///
    /// # Arguments
    ///
    /// * `size` - The new terminal size
    fn handle_resize_command(&mut self, size: Size) {
        self.terminal_size = size;

        // Reserve 2 rows for status and message bars, give the rest to the view
        self.view.resize(Size {
            height: size.height.saturating_sub(2),
            width: size.width,
        });

        // All bars get full width, 1 row height
        let bar_size = Size {
            height: 1,
            width: size.width,
        };
        self.message_bar.resize(bar_size);
        self.command_bar.resize(bar_size);
        self.status_bar.resize(bar_size);
    }

    /// Handles quit commands with protection for unsaved changes
    ///
    /// This method implements a safety mechanism for quitting with unsaved changes:
    /// - If no changes exist, quit immediately
    /// - If changes exist, require QUIT_TIMES consecutive quit attempts
    /// - Show a warning message indicating how many more quit attempts are needed
    ///
    /// This prevents accidental data loss while still allowing users to force quit.
    #[allow(clippy::arithmetic_side_effects)]
    fn handle_quit_command(&mut self) {
        if !self.view.get_status().is_modified || self.quit_times + 1 == QUIT_TIMES {
            self.should_quit = true;
        } else if self.view.get_status().is_modified {
            self.quit_times += 1;
            self.update_message(&format!(
                "WARNING! File has unsaved changes. Press Ctrl-Q {} more times to quit.",
                QUIT_TIMES - self.quit_times
            ));
        }
    }

    /// Resets the quit counter and clears any quit-related warnings
    ///
    /// This method is called whenever the user performs any action other than quitting,
    /// resetting the quit attempt counter to ensure they need to perform the full
    /// sequence of quit attempts again.
    fn reset_quit_times(&mut self) {
        if self.quit_times > 0 {
            self.quit_times = 0;
            self.update_message("");
        }
    }

    /// Handles save commands, determining whether to save directly or prompt for filename
    ///
    /// If a file is already loaded (has a filename), save directly to that file.
    /// Otherwise, enter save prompt mode to ask the user for a filename.
    fn handle_save_command(&mut self) {
        if self.view.is_file_loaded() {
            self.save(None);
        } else {
            self.set_prompt(PromptType::Save);
        }
    }
    /// Processes commands during save prompt mode
    ///
    /// In save mode, the editor handles:
    /// - Dismiss (Escape): Cancel the save operation
    /// - InsertNewline (Enter): Confirm save with the entered filename
    /// - Edit commands: Modify the filename being entered
    /// - Other commands are ignored
    ///
    /// # Arguments
    ///
    /// * `command` - The command to process
    fn process_command_during_save(&mut self, command: Command) {
        match command {
            System(Dismiss) => {
                self.set_prompt(PromptType::None);
                self.update_message("Save aborted.");
            },
            Edit(InsertNewline) => {
                let file_name = self.command_bar.value();
                self.save(Some(&file_name));
                self.set_prompt(PromptType::None);
            },
            Edit(edit_command) => self.command_bar.handle_edit_command(edit_command),
            _ => {}, // Ignore other commands during save prompt
        }
    }

    /// Saves the current document
    ///
    /// This method handles both "Save" and "Save As" operations:
    /// - If a filename is provided, save to that file (Save As)
    /// - If no filename is provided, save to the current file (Save)
    ///
    /// Displays appropriate success or error messages based on the result.
    ///
    /// # Arguments
    ///
    /// * `file_name` - Optional filename for "Save As" operation
    fn save(&mut self, file_name: Option<&str>) {
        let result = if let Some(name) = file_name {
            self.view.save_as(name)
        } else {
            self.view.save()
        };

        if result.is_ok() {
            self.update_message("File saved successfully.");
        } else {
            self.update_message("Error writing file!");
        }
    }

    /// Processes commands during search prompt mode
    ///
    /// In search mode, the editor handles:
    /// - Dismiss (Escape): Cancel search and restore previous position
    /// - InsertNewline (Enter): Exit search mode but keep current position
    /// - Edit commands: Modify the search query and update search results
    /// - Move Right/Down: Navigate to next search match
    /// - Move Up/Left: Navigate to previous search match
    /// - Other commands are ignored
    ///
    /// # Arguments
    ///
    /// * `command` - The command to process
    fn process_command_during_search(&mut self, command: Command) {
        match command {
            System(Dismiss) => {
                self.set_prompt(PromptType::None);
                self.view.dismiss_search();
            },
            Edit(InsertNewline) => {
                self.set_prompt(PromptType::None);
                self.view.exit_search();
            },
            Edit(edit_command) => {
                self.command_bar.handle_edit_command(edit_command);
                let query = self.command_bar.value();
                self.view.search(&query);
            },
            Move(Right | Down) => {
                self.view.search_next();
            },
            Move(Up | Left) => {
                self.view.search_prev();
            },
            _ => {}, // Ignore other commands during search
        }
    }
    /// Updates the message displayed in the message bar
    ///
    /// # Arguments
    ///
    /// * `new_message` - The message to display
    fn update_message(&mut self, new_message: &str) {
        self.message_bar.update_message(new_message);
    }

    /// Returns true if the editor is currently in any prompt mode
    fn in_prompt(&self) -> bool {
        !self.prompt_type.is_none()
    }

    /// Sets the editor to a specific prompt mode and configures the command bar
    ///
    /// This method handles the transition between different prompt states:
    /// - Save mode: Shows "Save as: " prompt
    /// - Search mode: Shows search prompt and enters search mode in the view
    /// - None mode: Exits prompt mode and marks message bar for redraw
    ///
    /// The command bar value is always cleared when entering a new prompt mode.
    ///
    /// # Arguments
    ///
    /// * `prompt_type` - The prompt mode to enter
    fn set_prompt(&mut self, prompt_type: PromptType) {
        match prompt_type {
            PromptType::Save => self.command_bar.set_prompt("Save as: "),
            PromptType::Search => {
                self.view.enter_search();
                self.command_bar
                    .set_prompt("Search (Esc to cancel, Arrows to navigate): ");
            },
            PromptType::None => self.message_bar.set_needs_redraw(true),
        }
        self.command_bar.clear_value();
        self.prompt_type = prompt_type;
    }
}

/// Drop implementation for Editor to ensure clean terminal shutdown
///
/// This implementation ensures that the terminal is properly restored to its
/// original state when the Editor is dropped, whether due to normal program
/// termination or a panic. If the editor is quitting normally, it also prints
/// a goodbye message.
impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::print("Goodbye.\r\n");
        }
    }
}
