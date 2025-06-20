mod attribute;
use super::{AnnotatedString, Position, Size};
use attribute::Attribute;
use std::io::{Error, Write, stdout};

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

/// Represents the Terminal.
/// Edge Case for platforms where `usize` < `u16`:
/// Regardless of the actual size of the Terminal, this representation
/// only spans over at most `usize::MAX` or `u16::size` rows/columns, whichever is smaller.
/// Each size returned truncates to min(`usize::MAX`, `u16::MAX`)
/// And should you attempt to set the caret out of these bounds, it will also be truncated.
pub struct Terminal;

impl Terminal {
    /// Initializes the terminal by enabling raw mode, entering the alternate screen,
    /// disabling line wrap, and clearing the screen.
    ///
    /// # Returns
    ///
    /// A result indicating success or failure.
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::enter_alternate_screen()?;
        Self::disable_line_wrap()?;
        Self::clear_screen()?;
        Self::execute()?;
        Ok(())
    }

    /// Terminates the terminal by leaving the alternate screen, enabling line wrap,
    /// showing the caret, and disabling raw mode.
    ///
    /// # Returns
    ///
    /// A result indicating success or failure.
    pub fn terminate() -> Result<(), Error> {
        Self::leave_alternate_screen()?;
        Self::enable_line_wrap()?;
        Self::show_caret()?;
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    /// Clears the entire screen.
    ///
    /// # Returns
    ///
    /// A result indicating success or failure.
    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    /// Clears the current line.
    ///
    /// # Returns
    ///
    /// A result indicating success or failure.
    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    /// Moves the caret to the given Position.
    /// # Arguments
    /// * `Position` - the  `Position`to move the caret to. Will be truncated to `u16::MAX` if bigger.
    pub fn move_caret_to(position: Position) -> Result<(), Error> {
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        Self::queue_command(MoveTo(position.col as u16, position.row as u16))?;
        Ok(())
    }

    /// Enters the alternate screen.
    ///
    /// # Returns
    ///
    /// A result indicating success or failure.
    pub fn enter_alternate_screen() -> Result<(), Error> {
        Self::queue_command(EnterAlternateScreen)?;
        Ok(())
    }

    /// Leaves the alternate screen.
    ///
    /// # Returns
    ///
    /// A result indicating success or failure.
    pub fn leave_alternate_screen() -> Result<(), Error> {
        Self::queue_command(LeaveAlternateScreen)?;
        Ok(())
    }

    /// Hides the caret.
    ///
    /// # Returns
    ///
    /// A result indicating success or failure.
    pub fn hide_caret() -> Result<(), Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }

    /// Shows the caret.
    ///
    /// # Returns
    ///
    /// A result indicating success or failure.
    pub fn show_caret() -> Result<(), Error> {
        Self::queue_command(Show)?;
        Ok(())
    }

    /// Disables line wrapping.
    ///
    /// # Returns
    ///
    /// A result indicating success or failure.
    pub fn disable_line_wrap() -> Result<(), Error> {
        Self::queue_command(DisableLineWrap)?;
        Ok(())
    }

    /// Enables line wrapping.
    ///
    /// # Returns
    ///
    /// A result indicating success or failure.
    pub fn enable_line_wrap() -> Result<(), Error> {
        Self::queue_command(EnableLineWrap)?;
        Ok(())
    }

    /// Sets the title of the terminal window.
    ///
    /// # Arguments
    ///
    /// * `title` - The title to set.
    ///
    /// # Returns
    ///
    /// A result indicating success or failure.
    pub fn set_title(title: &str) -> Result<(), Error> {
        Self::queue_command(SetTitle(title))?;
        Ok(())
    }

    /// Prints a string to the terminal.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to print.
    ///
    /// # Returns
    ///
    /// A result indicating success or failure.
    pub fn print(string: &str) -> Result<(), Error> {
        Self::queue_command(Print(string))?;
        Ok(())
    }

    /// Prints a row of text at the specified position.
    ///
    /// # Arguments
    ///
    /// * `row` - The row position.
    /// * `line_text` - The text to print.
    ///
    /// # Returns
    ///
    /// A result indicating success or failure.
    pub fn print_row(row: usize, line_text: &str) -> Result<(), Error> {
        Self::move_caret_to(Position { col: 0, row })?;
        Self::clear_line()?;
        Self::print(line_text)?;
        Ok(())
    }

    pub fn print_annotated_row(
        row: usize,
        annotated_string: &AnnotatedString,
    ) -> Result<(), Error> {
        Self::move_caret_to(Position { col: 0, row })?;
        Self::clear_line()?;
        annotated_string
            .into_iter()
            .try_for_each(|part| -> Result<(), Error> {
                if let Some(annotation_type) = part.annotation_type {
                    let attribute: Attribute = annotation_type.into();
                    Self::set_attribute(&attribute)?;
                }
                Self::print(part.string)?;
                Self::reset_color()?;
                Ok(())
            })?;
        Ok(())
    }

    fn set_attribute(attribute: &Attribute) -> Result<(), Error> {
        if let Some(foreground_color) = attribute.foreground {
            Self::queue_command(SetForegroundColor(foreground_color))?;
        }
        if let Some(background_color) = attribute.background {
            Self::queue_command(SetBackgroundColor(background_color))?;
        }
        Ok(())
    }

    fn reset_color() -> Result<(), Error> {
        Self::queue_command(ResetColor)?;
        Ok(())
    }

    /// Prints an inverted row of text at the specified position.
    ///
    /// # Arguments
    ///
    /// * `row` - The row position.
    /// * `line_text` - The text to print.
    ///
    /// # Returns
    ///
    /// A result indicating success or failure.
    pub fn print_inverted_row(row: usize, line_text: &str) -> Result<(), Error> {
        let width = Self::size()?.width;
        Self::print_row(
            row,
            &format!("{Reverse}{:width$.width$}{Reset}", line_text,),
        )
    }

    /// Returns the current size of this Terminal.
    /// Edge Case for systems with `usize` < `u16`:
    /// * A `Size` representing the terminal size. Any coordinate `z` truncated to `usize` if
    /// `usize` < `z` < `u16`
    pub fn size() -> Result<Size, Error> {
        let (width_u16, height_u16) = size()?;
        #[allow(clippy::as_conversions)]
        let width = width_u16 as usize;
        #[allow(clippy::as_conversions)]
        let height = height_u16 as usize;
        Ok(Size { width, height })
    }

    /// Executes all queued commands.
    ///
    /// # Returns
    ///
    /// A result indicating success or failure.
    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }

    /// Queues a command to be executed.
    ///
    /// # Arguments
    ///
    /// * `command` - The command to queue.
    ///
    /// # Returns
    ///
    /// A result indicating success or failure.
    fn queue_command<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }
}
