use crate::prelude::*;
use std::{
    env,
    io::Error,
    panic::{set_hook, take_hook},
};

use crossterm::event::{Event, KeyEvent, KeyEventKind, read};

mod annotated_string;
mod annotation;
mod annotation_type;
mod command;
mod document_status;
mod file_type;
mod line;
mod terminal;
mod ui_components;

use self::{
    annotated_string::AnnotatedString,
    annotation::Annotation,
    annotation_type::AnnotationType,
    command::{
        Command::{self, Edit, Move, System},
        Edit::InsertNewline,
        Move::{Down, Left, Right, Up},
        System::{Dismiss, Quit, Resize, Save, Search},
    },
    document_status::DocumentStatus,
    file_type::FileType,
    line::Line,
    terminal::Terminal,
    ui_components::{CommandBar, MessageBar, StatusBar, UIComponent, View},
};

const QUIT_TIMES: u8 = 3;

#[derive(Eq, PartialEq, Default)]
enum PromptType {
    Search,
    Save,
    #[default]
    None,
}

impl PromptType {
    fn is_none(&self) -> bool {
        *self == Self::None
    }
}

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    view: View,
    status_bar: StatusBar,
    message_bar: MessageBar,
    command_bar: CommandBar,
    prompt_type: PromptType,
    terminal_size: Size,
    title: String,
    quit_times: u8,
}
impl Editor {
    pub fn new() -> Result<Self, Error> {
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));

        Terminal::initialize()?;
        let mut editor = Self::default();
        let size = Terminal::size().unwrap_or_default();
        editor.handle_resize_command(size);
        editor.update_message("HELP: Ctrl-F = find | Ctrl-S = save | Ctrl-Q = quit");

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
    fn refresh_screen(&mut self) {
        let Size { height, width } = self.terminal_size;

        if height == 0 || width == 0 {
            return;
        }

        let bottom_bar_row = height.saturating_sub(1);
        let _ = Terminal::hide_caret();

        if self.in_prompt() {
            self.command_bar.render(bottom_bar_row);
        } else {
            self.message_bar.render(bottom_bar_row);
        }

        if height > 1 {
            self.status_bar.render(height.saturating_sub(2));
        }

        if height > 2 {
            self.view.render(0);
        }

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

    pub fn refresh_status(&mut self) {
        let status = self.view.get_status();
        let title = format!("{} - {NAME}", status.file_name);
        self.status_bar.update_status(status);
        if title != self.title && matches!(Terminal::set_title(&title), Ok(())) {
            self.title = title;
        }
    }
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

    fn process_command(&mut self, command: Command) {
        if let System(Resize(size)) = command {
            self.handle_resize_command(size);
            return;
        }

        match self.prompt_type {
            PromptType::Save => self.process_command_during_save(command),
            PromptType::Search => self.process_command_during_search(command),
            PromptType::None => self.process_command_no_prompt(command),
        }
    }

    fn process_command_no_prompt(&mut self, command: Command) {
        if matches!(command, System(Quit)) {
            self.handle_quit_command();
            return;
        }

        self.reset_quit_times();

        match command {
            System(Search) => self.set_prompt(PromptType::Search),
            System(Save) => self.handle_save_command(),
            Edit(edit_command) => self.view.handle_edit_command(edit_command),
            Move(move_command) => self.view.handle_move_command(move_command),
            System(_) => {},
        }
    }
    fn handle_resize_command(&mut self, size: Size) {
        self.terminal_size = size;

        self.view.resize(Size {
            height: size.height.saturating_sub(2),
            width: size.width,
        });

        let bar_size = Size {
            height: 1,
            width: size.width,
        };
        self.message_bar.resize(bar_size);
        self.command_bar.resize(bar_size);
        self.status_bar.resize(bar_size);
    }

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

    fn reset_quit_times(&mut self) {
        if self.quit_times > 0 {
            self.quit_times = 0;
            self.update_message("");
        }
    }

    fn handle_save_command(&mut self) {
        if self.view.is_file_loaded() {
            self.save(None);
        } else {
            self.set_prompt(PromptType::Save);
        }
    }
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
            _ => {},
        }
    }

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
            _ => {},
        }
    }
    fn update_message(&mut self, new_message: &str) {
        self.message_bar.update_message(new_message);
    }

    fn in_prompt(&self) -> bool {
        !self.prompt_type.is_none()
    }

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

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::print("Goodbye.\r\n");
        }
    }
}
