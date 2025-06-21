//! # UI Component Trait Module
//!
//! This module defines the UIComponent trait which provides a common interface
//! for all UI components in the Hecto editor. It establishes a consistent
//! pattern for rendering, resizing, and managing the redraw state of UI elements.

use crate::prelude::*;

use std::io::Error;

/// Common interface for all UI components in the editor
///
/// UIComponent defines the basic contract that all UI elements must implement
/// to participate in the editor's rendering system. It provides a framework
/// for efficient rendering with dirty state tracking and consistent resize handling.
///
/// ## Design Pattern
///
/// The trait follows a dirty-state pattern where components track whether they
/// need to be redrawn. This avoids unnecessary terminal operations and improves
/// performance by only redrawing components that have actually changed.
///
/// ## Implementation Guide
///
/// Components should:
/// 1. Implement `draw()` to perform actual terminal rendering
/// 2. Implement `set_size()` to handle dimension changes
/// 3. Implement dirty state tracking via `set_needs_redraw()` and `needs_redraw()`
/// 4. Call `set_needs_redraw(true)` whenever content changes
pub trait UIComponent {
    /// Sets whether the component needs to be redrawn
    ///
    /// This method controls the dirty state of the component. Setting to `true`
    /// indicates that the component's visual representation has changed and
    /// needs to be updated on the next render cycle.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the component needs redrawing
    fn set_needs_redraw(&mut self, value: bool);

    /// Returns whether the component needs to be redrawn
    ///
    /// # Returns
    ///
    /// `true` if the component should be redrawn, `false` otherwise
    fn needs_redraw(&self) -> bool;

    /// Resizes the component and marks it for redraw
    ///
    /// This method provides a default implementation that updates the component's
    /// size and automatically marks it as needing a redraw. Components can override
    /// this if they need custom resize behavior.
    ///
    /// # Arguments
    ///
    /// * `size` - The new dimensions for the component
    fn resize(&mut self, size: Size) {
        self.set_size(size);
        self.set_needs_redraw(true);
    }

    /// Sets the component's dimensions
    ///
    /// This method should be implemented to store the component's size and
    /// perform any necessary layout calculations. It should not trigger
    /// rendering directly.
    ///
    /// # Arguments
    ///
    /// * `size` - The new dimensions for the component
    fn set_size(&mut self, size: Size);

    /// Renders the component if it needs redrawing
    ///
    /// This method provides the main rendering logic that checks the dirty state
    /// and calls the component's `draw()` method if needed. It handles error
    /// conditions appropriately and clears the dirty state on successful rendering.
    ///
    /// # Arguments
    ///
    /// * `origin_row` - The row where the component should start rendering
    fn render(&mut self, origin_row: RowIdx) {
        if self.needs_redraw() {
            if let Err(err) = self.draw(origin_row) {
                #[cfg(debug_assertions)]
                {
                    panic!("Could not render component: {err:?}");
                }
                #[cfg(not(debug_assertions))]
                {
                    let _ = err;
                }
            } else {
                self.set_needs_redraw(false);
            }
        }
    }

    /// Performs the actual drawing operations for the component
    ///
    /// This method must be implemented by each component to define how it
    /// renders itself to the terminal. It should use the Terminal interface
    /// to output content at the specified row.
    ///
    /// # Arguments
    ///
    /// * `origin_row` - The row where the component should start rendering
    ///
    /// # Returns
    ///
    /// `Ok(())` on successful rendering, or an `Error` if rendering fails
    ///
    /// # Errors
    ///
    /// This method should return an error if any terminal operations fail
    /// during the rendering process.
    fn draw(&mut self, origin_row: RowIdx) -> Result<(), Error>;
}
