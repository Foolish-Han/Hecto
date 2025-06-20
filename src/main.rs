//! # Hecto Text Editor
//!
//! A terminal-based text editor written in Rust, designed to be simple, efficient, and fun to use.
//!
//! Hecto provides essential features for text editing including:
//! - Opening, editing, and saving files
//! - Text search functionality with highlighting
//! - Cursor navigation and text manipulation
//! - Terminal-based user interface with status and message bars
//!
//! ## Key Features
//! - **Safe and Fast**: Built with Rust's safety guarantees and zero-cost abstractions
//! - **Unicode Support**: Proper handling of Unicode graphemes and width calculations
//! - **Terminal Integration**: Full terminal control with alternate screen and raw mode
//! - **Search Functionality**: Forward and backward search with match highlighting
//! - **Modular Architecture**: Clean separation of concerns with well-defined modules
//!
//! ## Usage
//! Run the editor with an optional filename:
//! ```bash
//! cargo run [filename]
//! ```
//!
//! ## Key Bindings
//! - `Ctrl+Q`: Quit the editor
//! - `Ctrl+S`: Save the current file
//! - `Ctrl+F`: Search within the document
//! - Arrow keys: Navigate the cursor
//! - `Page Up/Down`: Navigate by pages
//! - `Home/End`: Navigate to line start/end

#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::print_stdout,
    clippy::arithmetic_side_effects,
    clippy::integer_division,
    clippy::as_conversions
)]

mod editor;
use editor::Editor;

/// Entry point for the Hecto text editor application.
///
/// This function initializes a new Editor instance and starts the main event loop.
/// The editor will handle terminal initialization, file loading (if specified as a command-line argument),
/// and user input processing until the user quits.
///
/// # Panics
///
/// The application will panic if the editor fails to initialize due to terminal setup errors
/// or other system-level failures during startup.
fn main() {
    Editor::new().unwrap().run();
}
