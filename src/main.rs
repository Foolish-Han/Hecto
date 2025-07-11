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
mod prelude;
use prelude::*;

fn main() {
    let _ = setup_logger();
    Editor::new().unwrap().run();
}
