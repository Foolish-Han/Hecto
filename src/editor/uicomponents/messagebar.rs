use std::{
    io::Error,
    time::{Duration, Instant},
};

use super::{super::{Size, Terminal}, UIComponent};

const DEFAULT_DURATION: Duration = Duration::new(5, 0);

/// Represents a message to be displayed in the message bar.
struct Message {
    /// The text of the message.
    text: String,
    /// The time the message was created.
    time: Instant,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            text: String::new(),
            time: Instant::now(),
        }
    }
}

impl Message {
    /// Checks if the message has expired.
    ///
    /// # Returns
    ///
    /// `true` if the message has expired, `false` otherwise.
    fn is_expired(&self) -> bool {
        Instant::now().duration_since(self.time) > DEFAULT_DURATION
    }
}

/// Represents the message bar in the editor.
#[derive(Default)]
pub struct MessageBar {
    /// The current message being displayed.
    current_message: Message,
    /// Indicates whether the message bar needs to be redrawn.
    needs_redraw: bool,
    /// Indicates whether the message bar was cleared after the message expired.
    cleared_after_expiry: bool,
}

impl MessageBar {
    /// Updates the message displayed in the message bar.
    ///
    /// # Arguments
    ///
    /// * `new_message` - The new message to display.
    pub fn update_message(&mut self, new_message: &str) {
        self.current_message = Message {
            text: new_message.to_string(),
            time: Instant::now(),
        };
        self.cleared_after_expiry = false;
        self.set_needs_redraw(true);
    }
}

impl UIComponent for MessageBar {
    fn set_needs_redraw(&mut self, value: bool) {
        self.needs_redraw = value;
    }

    fn needs_redraw(&self) -> bool {
        (!self.cleared_after_expiry && self.current_message.is_expired()) || self.needs_redraw
    }

    fn set_size(&mut self, _: Size) {}

    fn draw(&mut self, origin_y: usize) -> Result<(), Error> {
        if self.current_message.is_expired() {
            self.cleared_after_expiry = true;
        }

        let message = if self.current_message.is_expired() {
            ""
        } else {
            &self.current_message.text
        };

        Terminal::print_row(origin_y, message)?;
        Ok(())
    }
}
