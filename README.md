# Hecto Editor 🎉

Welcome to Hecto, a playful and powerful text editor written in Rust! This README will guide you through the project's architecture, its purpose, and how to get started with building, running, and contributing to Hecto. 😊

## Overview

Hecto is a terminal-based text editor designed to be simple, efficient, and fun to use. It provides essential features for text editing, such as opening, editing, and saving files, as well as searching within the text. Hecto is built with Rust, leveraging its safety and performance features to create a reliable and fast editor. 🚀

## Project Architecture

Hecto's architecture is modular, with each module responsible for a specific aspect of the editor's functionality. Here's a high-level overview of the main modules and their responsibilities:

- `src/editor.rs`: The main editor application, handling the overall functionality and event loop.
- `src/editor/command.rs`: Defines the various commands (move, edit, system) that the editor can execute.
- `src/editor/commandbar.rs`: Manages the command bar, where users can input commands.
- `src/editor/documentstatus.rs`: Represents the status of a document, including line count and modification status.
- `src/editor/line.rs`: Handles the representation and manipulation of individual lines of text.
- `src/editor/messagebar.rs`: Manages the message bar, displaying messages to the user.
- `src/editor/position.rs`: Defines the position type used to represent cursor positions.
- `src/editor/size.rs`: Represents the size of the terminal.
- `src/editor/statusbar.rs`: Manages the status bar, displaying information about the current document.
- `src/editor/terminal.rs`: Handles terminal input/output operations.
- `src/editor/uicomponent.rs`: Defines a trait for UI components, providing methods for rendering and resizing.
- `src/editor/view.rs`: Manages the view of the text buffer, including rendering and scrolling.

## Building and Running

To build and run Hecto, follow these steps:

1. Ensure you have Rust installed. If not, you can install it from [rust-lang.org](https://www.rust-lang.org/).
2. Clone the repository:
   ```sh
   git clone https://github.com/Foolish-Han/Hecto.git
   cd Hecto
   ```
3. Build the project:
   ```sh
   cargo build --release
   ```
4. Run the editor:
   ```sh
   cargo run --release
   ```

## Usage

Once you have Hecto running, you can use the following keybindings to interact with the editor:

- `Ctrl-F`: Find text within the document.
- `Ctrl-S`: Save the current document.
- `Ctrl-Q`: Quit the editor.
- Arrow keys: Move the cursor.
- `Enter`: Insert a newline.
- `Backspace`: Delete the character before the cursor.
- `Delete`: Delete the character under the cursor.

## Contributing

We welcome contributions to Hecto! If you'd like to contribute, please follow these guidelines:

1. Fork the repository and create a new branch for your feature or bugfix.
2. Write clear, concise, and professional comments in your code.
3. Ensure your code follows Rust's best practices and passes all tests.
4. Submit a pull request with a detailed description of your changes.

If you encounter any issues or have suggestions for improvements, please open an issue on GitHub. We appreciate your feedback and contributions!

## Learning Purpose

This project is for learning purposes and is based on the tutorial by Philipp Flenker. You can find the original tutorial at [https://philippflenker.com/](https://philippflenker.com/). 📚

Happy coding with Hecto! 🎉✨
