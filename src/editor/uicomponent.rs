use std::io::Error;

use super::Size;

/// A trait representing a UI component in the editor.
pub trait UIComponent {
    /// Marks this UI component as in need of redrawing (or not).
    ///
    /// # Arguments
    ///
    /// * `value` - `true` if the component needs to be redrawn, `false` otherwise.
    fn set_needs_redraw(&mut self, value: bool);

    /// Determines if a component needs to be redrawn or not.
    ///
    /// # Returns
    ///
    /// `true` if the component needs to be redrawn, `false` otherwise.
    fn needs_redraw(&self) -> bool;

    /// Updates the size and marks the component as needing redrawing.
    ///
    /// # Arguments
    ///
    /// * `size` - The new size of the component.
    fn resize(&mut self, size: Size) {
        self.set_size(size);
        self.set_needs_redraw(true);
    }

    /// Updates the size of the component.
    ///
    /// # Arguments
    ///
    /// * `size` - The new size of the component.
    fn set_size(&mut self, size: Size);

    /// Draws this component if it's visible and in need of redrawing.
    ///
    /// # Arguments
    ///
    /// * `origin_row` - The row to start drawing the component at.
    fn render(&mut self, origin_row: usize) {
        if self.needs_redraw() {
            match self.draw(origin_row) {
                Ok(()) => self.set_needs_redraw(false),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not render component: {err:?}");
                    }
                }
            }
        }
    }

    /// Method to actually draw the component.
    ///
    /// # Arguments
    ///
    /// * `origin_row` - The row to start drawing the component at.
    ///
    /// # Returns
    ///
    /// A result indicating success or failure.
    fn draw(&mut self, origin_row: usize) -> Result<(), Error>;
}
