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

/// The main entry point of the application.
///
/// This function initializes and runs the editor.
fn main() {
    Editor::new().unwrap().run();
}
