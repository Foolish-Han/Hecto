# Hecto Editor 🦀✨

A modern, terminal-based text editor written in Rust, designed to be simple, efficient, and powerful. Hecto combines the safety and performance of Rust with an intuitive editing experience and comprehensive Unicode support.

## 🌟 Features

### Core Editing
- **Unicode-Aware Text Editing**: Proper handling of Unicode grapheme clusters, multi-byte characters, and combining characters
- **Efficient File Operations**: Fast loading and saving of text files with proper encoding support  
- **Smart Cursor Movement**: Precise cursor positioning with support for complex Unicode text
- **Line-Based Operations**: Newline insertion, line splitting, and merging

### Search & Navigation
- **Interactive Search**: Forward and backward text search with real-time highlighting
- **Search Navigation**: Jump between search results with arrow keys
- **Match Highlighting**: Visual highlighting of search matches and current selection
- **Search State Management**: Return to original position when canceling search

### User Interface
- **Status Bar**: Real-time display of document information, cursor position, and file status
- **Message Bar**: Contextual messages and notifications with automatic expiration
- **Command Bar**: Interactive prompts for save operations and search queries
- **Welcome Screen**: Friendly welcome message for new sessions

### Terminal Integration
- **Full Terminal Control**: Alternate screen mode with proper terminal restoration
- **Responsive Layout**: Automatic adaptation to terminal resizing
- **Smooth Scrolling**: Efficient viewport management with horizontal and vertical scrolling
- **Cross-Platform**: Works on Linux, macOS, and Windows terminals

## 🏗️ Architecture

Hecto follows a modular, component-based architecture with clear separation of concerns:

### Core Components
- **`src/main.rs`**: Application entry point and initialization
- **`src/editor.rs`**: Main editor controller and event loop management
- **`src/editor/terminal/`**: Low-level terminal operations and attribute management
- **`src/editor/command/`**: Command system for processing user input
  - `edit.rs`: Text editing commands (insert, delete, newline)
  - `movecommand.rs`: Cursor movement commands
  - `system.rs`: System operations (save, quit, search, resize)

### Text Processing
- **`src/editor/line/`**: Unicode-aware line representation and manipulation
  - `textfragment.rs`: Text fragment processing with styling
  - `graphemewidth.rs`: Unicode grapheme cluster width calculations
- **`src/editor/annotatedstring/`**: Text annotation system for syntax highlighting
  - `annotation.rs`: Text annotation definitions
  - `annotationtype.rs`: Types of annotations (highlight, selection, etc.)
  - `annotationstringiterator.rs`: Efficient iteration over annotated text

### UI Components
- **`src/editor/uicomponents/view/`**: Main text editing view
  - `buffer.rs`: Text buffer management with file I/O
  - `location.rs`: Cursor position tracking
  - `searchinfo.rs`: Search state management
  - `fileinfo.rs`: File metadata handling
- **`src/editor/uicomponents/statusbar.rs`**: Document status display
- **`src/editor/uicomponents/messagebar.rs`**: Temporary message display
- **`src/editor/uicomponents/commandbar.rs`**: Interactive user input

### Supporting Types
- **`src/editor/position.rs`**: 2D coordinate system (row/column)
- **`src/editor/size.rs`**: Viewport dimensions
- **`src/editor/documentstatus.rs`**: Document metadata structure

## 🚀 Getting Started

### Prerequisites
- **Rust**: Install from [rust-lang.org](https://www.rust-lang.org/) (2024 edition)
- **Terminal**: Any modern terminal emulator with Unicode support

### Building and Running

1. **Clone the repository**:
   ```bash
   git clone https://github.com/Foolish-Han/Hecto.git
   cd Hecto
   ```

2. **Build the project**:
   ```bash
   cargo build --release
   ```

3. **Run the editor**:
   ```bash
   # Start with a new file
   cargo run --release

   # Open an existing file
   cargo run --release filename.txt
   ```

### Development
```bash
# Run in development mode
cargo run [filename]

# Run tests
cargo test

# Check code formatting
cargo fmt

# Run linter
cargo clippy
```

## ⌨️ Key Bindings

### File Operations
- **`Ctrl+S`**: Save current file (prompts for filename if new)
- **`Ctrl+Q`**: Quit editor (requires 3 consecutive presses if unsaved changes)

### Navigation
- **Arrow Keys**: Move cursor in all directions
- **`Page Up/Down`**: Navigate by viewport height
- **`Home`**: Move to beginning of line
- **`End`**: Move to end of line

### Editing
- **`Enter`**: Insert newline and move to next line
- **`Backspace`**: Delete character before cursor
- **`Delete`**: Delete character at cursor
- **Regular characters**: Insert at cursor position

### Search
- **`Ctrl+F`**: Enter search mode
- **`Escape`**: Cancel search and return to original position
- **`Enter`** (in search): Keep current position and exit search
- **`→/↓`** (in search): Find next match
- **`←/↑`** (in search): Find previous match

## 🛠️ Dependencies

```toml
[dependencies]
crossterm = "0.29.0"        # Cross-platform terminal manipulation
unicode-segmentation = "1.11.0"  # Unicode grapheme cluster support
unicode-width = "0.1.12"    # Unicode character width calculations
```

## 🎯 Project Goals

This project serves multiple purposes:

1. **Learning Rust**: Demonstrates advanced Rust concepts including error handling, lifetimes, and zero-cost abstractions
2. **Terminal Programming**: Shows modern terminal application development with proper Unicode support
3. **Software Architecture**: Illustrates clean, modular design with separation of concerns
4. **Text Processing**: Implements sophisticated Unicode-aware text handling

## 🤝 Contributing

Contributions are welcome! Please follow these guidelines:

1. **Fork the repository** and create a feature branch
2. **Write clear, documented code** following Rust best practices
3. **Add tests** for new functionality
4. **Update documentation** as needed
5. **Submit a pull request** with a detailed description

### Code Standards
- Follow `rustfmt` formatting
- Pass all `clippy` lints
- Include comprehensive documentation
- Write meaningful commit messages

## 📚 Learning Resources

This project is inspired by and builds upon:
- **Tutorial by Philipp Flenker**: [https://philippflenker.com/](https://philippflenker.com/)
- **The Rust Programming Language**: [https://doc.rust-lang.org/book/](https://doc.rust-lang.org/book/)
- **Unicode Standards**: Understanding proper text handling in modern applications

## 📄 License

This project is open source and available under the [MIT License](LICENSE).

---

**Happy coding with Hecto!** 🦀✨

*A text editor that grows with you, built with Rust's safety and performance in mind.*
