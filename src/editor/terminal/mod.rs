//! # Terminal Module
//!
//! This module provides a high-level interface for terminal operations in the Hecto editor.
//! It wraps the crossterm library functionality to provide editor-specific terminal
//! control operations.
//!
//! ## Features
//!
//! - Terminal initialization and cleanup
//! - Screen clearing and cursor management
//! - Text output with styling support
//! - Alternate screen buffer management
//! - Terminal size detection
//! - Line wrapping control
//!
//! ## Design
//!
//! The Terminal struct uses a static design pattern where all methods are associated
//! functions rather than instance methods. This simplifies the API since there's
//! typically only one terminal per application.

mod attribute;

use std::io::{Error, Write, stdout};

use attribute::Attribute;
use crossterm::{
    Command,
    cursor::{Hide, MoveTo, Show},
    queue,
    style::{
        Attribute::{Reset, Reverse},
        Print, ResetColor, SetBackgroundColor, SetForegroundColor,
    },
    terminal::{
        Clear, ClearType, DisableLineWrap, EnableLineWrap, EnterAlternateScreen,
        LeaveAlternateScreen, SetTitle, disable_raw_mode, enable_raw_mode, size,
    },
};

use super::{AnnotatedString, Position, Size};

/// Terminal interface for the Hecto editor
///
/// Terminal provides a high-level interface to terminal operations, abstracting
/// the complexities of terminal control and providing editor-specific functionality.
/// All methods are static since there's typically only one terminal per application.
pub struct Terminal;

impl Terminal {
    /// Initializes the terminal for editor use
    ///
    /// This method performs all necessary terminal setup operations:
    /// - Enables raw mode for direct key input handling
    /// - Enters alternate screen buffer to preserve user's terminal content
    /// - Disables line wrapping for better text display control
    /// - Clears the screen
    /// - Executes all queued terminal commands
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on successful initialization, or an `Error` if any
    /// terminal operation fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - Raw mode cannot be enabled
    /// - Alternate screen cannot be entered
    /// - Terminal commands fail to execute
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::enter_alternate_screen()?;
        Self::disable_line_wrap()?;
        Self::clear_screen()?;
        Self::execute()?;
        Ok(())
    }

    /// Terminates the terminal and restores it to normal state
    ///
    /// This method performs cleanup operations to restore the terminal:
    /// - Leaves alternate screen buffer
    /// - Re-enables line wrapping
    /// - Shows the cursor
    /// - Executes all queued commands
    /// - Disables raw mode
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on successful termination, or an `Error` if any
    /// terminal operation fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if terminal restoration operations fail.
    /// Note that some errors may be ignored in cleanup scenarios to prevent
    /// panic-during-panic situations.
    pub fn terminate() -> Result<(), Error> {
        Self::leave_alternate_screen()?;
        Self::enable_line_wrap()?;
        Self::show_caret()?;
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }
    /// Clears the entire terminal screen
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if the clear operation fails.
    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    /// Clears the current line where the cursor is positioned
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if the clear operation fails.
    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    /// Moves the cursor to the specified position
    ///
    /// # Arguments
    ///
    /// * `position` - The target position for the cursor
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if the move operation fails.
    pub fn move_caret_to(position: Position) -> Result<(), Error> {
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        Self::queue_command(MoveTo(position.col as u16, position.row as u16))?;
        Ok(())
    }

    /// Enters the alternate screen buffer
    ///
    /// This preserves the current terminal content and provides a clean
    /// workspace for the editor.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if the operation fails.
    pub fn enter_alternate_screen() -> Result<(), Error> {
        Self::queue_command(EnterAlternateScreen)?;
        Ok(())
    }

    /// Leaves the alternate screen buffer
    ///
    /// This restores the previous terminal content that was displayed
    /// before entering the alternate screen.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if the operation fails.
    pub fn leave_alternate_screen() -> Result<(), Error> {
        Self::queue_command(LeaveAlternateScreen)?;
        Ok(())
    }

    /// Hides the cursor from display
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if the operation fails.
    pub fn hide_caret() -> Result<(), Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }

    /// Shows the cursor in the terminal
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if the operation fails.
    pub fn show_caret() -> Result<(), Error> {
        Self::queue_command(Show)?;
        Ok(())
    }

    /// Disables automatic line wrapping
    ///
    /// This prevents the terminal from automatically wrapping long lines,
    /// giving the editor control over line display.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if the operation fails.
    pub fn disable_line_wrap() -> Result<(), Error> {
        Self::queue_command(DisableLineWrap)?;
        Ok(())
    }

    /// Enables automatic line wrapping
    ///
    /// This restores the terminal's default line wrapping behavior.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if the operation fails.
    pub fn enable_line_wrap() -> Result<(), Error> {
        Self::queue_command(EnableLineWrap)?;
        Ok(())
    }

    /// Sets the terminal window title
    ///
    /// # Arguments
    ///
    /// * `title` - The title to set for the terminal window
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if the operation fails.
    pub fn set_title(title: &str) -> Result<(), Error> {
        Self::queue_command(SetTitle(title))?;
        Ok(())
    }

    /// Prints a string to the terminal at the current cursor position
    ///
    /// # Arguments
    ///
    /// * `string` - The text to print
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if the print operation fails.
    pub fn print(string: &str) -> Result<(), Error> {
        Self::queue_command(Print(string))?;
        Ok(())
    }
    /// Prints text to a specific row, clearing the line first
    ///
    /// This method moves the cursor to the beginning of the specified row,
    /// clears the entire line, and then prints the provided text.
    ///
    /// # Arguments
    ///
    /// * `row` - The row number (0-based) where to print the text
    /// * `line_text` - The text to print on the line
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if any operation fails.
    pub fn print_row(row: usize, line_text: &str) -> Result<(), Error> {
        Self::move_caret_to(Position { col: 0, row })?;
        Self::clear_line()?;
        Self::print(line_text)?;
        Ok(())
    }

    /// Prints an annotated string to a specific row with styling
    ///
    /// This method renders text with different styling attributes based on
    /// the annotations in the AnnotatedString. It handles search highlighting
    /// and other visual effects.
    ///
    /// # Arguments
    ///
    /// * `row` - The row number (0-based) where to print the text
    /// * `annotated_string` - The annotated string with styling information
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if any operation fails.
    pub fn print_annotated_row(
        row: usize,
        annotated_string: &AnnotatedString,
    ) -> Result<(), Error> {
        Self::move_caret_to(Position { col: 0, row })?;
        Self::clear_line()?;

        // Iterate through each part of the annotated string
        annotated_string
            .into_iter()
            .try_for_each(|part| -> Result<(), Error> {
                // Apply styling if this part has an annotation
                if let Some(annotation_type) = part.annotation_type {
                    let attribute: Attribute = annotation_type.into();
                    Self::set_attribute(&attribute)?;
                }
                // Print the text part
                Self::print(part.string)?;
                // Reset colors after styled text
                Self::reset_color()?;
                Ok(())
            })?;
        Ok(())
    }

    /// Applies display attributes (colors) to subsequent text output
    ///
    /// # Arguments
    ///
    /// * `attribute` - The attribute containing color information to apply
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if the operation fails.
    fn set_attribute(attribute: &Attribute) -> Result<(), Error> {
        if let Some(foreground_color) = attribute.foreground {
            Self::queue_command(SetForegroundColor(foreground_color))?;
        }
        if let Some(background_color) = attribute.background {
            Self::queue_command(SetBackgroundColor(background_color))?;
        }
        Ok(())
    }

    /// Resets all color attributes to terminal defaults
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if the operation fails.
    fn reset_color() -> Result<(), Error> {
        Self::queue_command(ResetColor)?;
        Ok(())
    }

    /// Prints text to a specific row with inverted colors (reverse video)
    ///
    /// This method is commonly used for status bars and other UI elements
    /// that need to stand out from the main text content. The text is
    /// padded to fill the entire terminal width.
    ///
    /// # Arguments
    ///
    /// * `row` - The row number (0-based) where to print the text
    /// * `line_text` - The text to print with inverted colors
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if any operation fails.
    pub fn print_inverted_row(row: usize, line_text: &str) -> Result<(), Error> {
        let width = Self::size()?.width;
        Self::print_row(
            row,
            &format!("{Reverse}{:width$.width$}{Reset}", line_text,),
        )
    }

    /// Gets the current terminal size
    ///
    /// # Returns
    ///
    /// Returns a `Result<Size, Error>` containing the terminal dimensions
    /// on success, or an `Error` if the size cannot be determined.
    pub fn size() -> Result<Size, Error> {
        let (width_u16, height_u16) = size()?;
        #[allow(clippy::as_conversions)]
        let width = width_u16 as usize;
        #[allow(clippy::as_conversions)]
        let height = height_u16 as usize;
        Ok(Size { width, height })
    }

    /// Executes all queued terminal commands
    ///
    /// This method flushes the stdout buffer, causing all previously queued
    /// terminal commands to be executed immediately.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if the flush operation fails.
    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }

    /// Queues a terminal command for later execution
    ///
    /// This is a private helper method that queues commands using the crossterm
    /// library. Commands are not executed immediately but are batched for
    /// efficiency.
    ///
    /// # Arguments
    ///
    /// * `command` - The terminal command to queue
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if the command cannot be queued.
    fn queue_command<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }
}
