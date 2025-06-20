//! # Message Bar Component
//!
//! This module implements the message bar component that displays temporary
//! informational messages to the user. Messages automatically expire after
//! a configured duration and are cleared from the display.

use std::{
    io::Error,
    time::{Duration, Instant},
};

use super::{
    super::{Size, Terminal},
    UIComponent,
};

/// Default duration for message display before automatic expiration
const DEFAULT_DURATION: Duration = Duration::new(5, 0);

/// Represents a single message with its content and timestamp
///
/// Message stores both the text content and the time when it was created,
/// allowing for automatic expiration after a specified duration.
struct Message {
    /// The message text to display
    text: String,
    /// When the message was created
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
    /// Checks if the message has expired
    ///
    /// # Returns
    ///
    /// `true` if the message is older than the default duration, `false` otherwise
    fn is_expired(&self) -> bool {
        Instant::now().duration_since(self.time) > DEFAULT_DURATION
    }
}

/// Message bar component for displaying temporary user messages
///
/// The MessageBar displays informational messages that automatically expire
/// after a set duration. It's used for status updates, error messages,
/// help text, and other temporary notifications that don't require user
/// interaction to dismiss.
///
/// ## Behavior
///
/// - Messages are displayed for a fixed duration (5 seconds by default)
/// - Expired messages are automatically cleared from the display
/// - New messages replace existing ones immediately
/// - The component optimizes redraws to only occur when needed
#[derive(Default)]
pub struct MessageBar {
    /// The currently displayed message
    current_message: Message,
    /// Whether the component needs redrawing
    needs_redraw: bool,
    /// Whether an expired message has been cleared (prevents redundant clears)
    cleared_after_expiry: bool,
}

impl MessageBar {
    /// Updates the message bar with new content
    ///
    /// This method sets a new message and resets the expiration timer.
    /// The message bar will be marked for redraw to display the new content.
    ///
    /// # Arguments
    ///
    /// * `new_message` - The text to display in the message bar
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

    /// Determines if the message bar needs redrawing
    ///
    /// The message bar needs redrawing in two cases:
    /// 1. It has been explicitly marked for redraw
    /// 2. The current message has expired and hasn't been cleared yet
    ///
    /// This ensures that expired messages are automatically removed from display.
    ///
    /// # Returns
    ///
    /// `true` if the component should be redrawn, `false` otherwise
    fn needs_redraw(&self) -> bool {
        (!self.cleared_after_expiry && self.current_message.is_expired()) || self.needs_redraw
    }

    /// Sets the component size (no-op for message bar)
    ///
    /// The message bar doesn't need to track its size since it always
    /// uses the full width of the terminal for rendering.
    fn set_size(&mut self, _: Size) {}

    /// Renders the message bar content
    ///
    /// This method displays the current message if it hasn't expired,
    /// or clears the line if the message has expired. It handles the
    /// automatic expiration logic and marks expired messages as cleared.
    ///
    /// # Arguments
    ///
    /// * `origin_y` - The row where the message bar should be rendered
    ///
    /// # Returns
    ///
    /// `Ok(())` on successful rendering, or an `Error` if terminal operations fail
    fn draw(&mut self, origin_y: usize) -> Result<(), Error> {
        // Check if the message has expired and mark it as cleared
        if self.current_message.is_expired() {
            self.cleared_after_expiry = true;
        }

        // Display the message text or empty string if expired
        let message = if self.current_message.is_expired() {
            ""
        } else {
            &self.current_message.text
        };

        Terminal::print_row(origin_y, message)?;
        Ok(())
    }
}
