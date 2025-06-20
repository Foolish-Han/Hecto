//! # UI Components Module
//!
//! This module contains all the user interface components used in the Hecto editor.
//! It provides a set of reusable UI elements that follow a consistent design pattern
//! and work together to create the complete editor interface.
//!
//! ## Architecture
//!
//! All UI components implement the `UIComponent` trait, which provides:
//! - Consistent rendering lifecycle management
//! - Efficient dirty-state tracking for performance
//! - Standardized resize handling
//! - Error handling for rendering operations
//!
//! ## Components
//!
//! - **View**: The main text editing area that displays document content
//! - **StatusBar**: Shows document information and cursor position
//! - **MessageBar**: Displays informational messages to the user
//! - **CommandBar**: Handles user input during prompts (save, search)
//!
//! ## Usage Pattern
//!
//! Components are typically used in this pattern:
//! 1. Create and configure the component
//! 2. Handle resize events by calling `resize()`
//! 3. Update component content and call `set_needs_redraw(true)`
//! 4. Call `render()` during the display refresh cycle

mod commandbar;
mod messagebar;
mod statusbar;
mod uicomponent;
mod view;

pub use commandbar::CommandBar;
pub use messagebar::MessageBar;
pub use statusbar::StatusBar;
pub use uicomponent::UIComponent;
pub use view::View;
