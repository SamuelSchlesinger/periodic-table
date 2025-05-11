# Periodic Table TUI

An interactive periodic table of elements built with Rust using the Ratatui and Crossterm libraries.

## Features

- Interactive periodic table with color-coded elements based on element categories
- Element selection with detailed information display
- Keyboard navigation (arrow keys) and mouse support
- Visual highlighting of selected elements
- Complete information for all elements including:
  - Atomic number
  - Symbol
  - Name
  - Atomic mass
  - Category
  - Description

## Installation

### Prerequisites

- Rust compiler and Cargo (2021 edition or later)

### Building from source

```bash
# Clone the repository
git clone https://github.com/yourusername/periodic-table.git
cd periodic-table

# Build and run the project
cargo build --release
cargo run --release
```

## Usage

- Use **arrow keys** to navigate the periodic table
- Press **Enter** to select an element (detailed information will appear at the bottom)
- Use **mouse clicks** to select elements directly
- Press **q** to exit the application

## Element Categories

Elements are color-coded by their category:

- Alkali Metals: Red
- Alkaline Earth Metals: Orange
- Lanthanides: Light Purple
- Actinides: Purple
- Transition Metals: Yellow
- Post-transition Metals: Light Blue
- Metalloids: Light Green
- Nonmetals: Green
- Halogens: Cyan
- Noble Gases: Pink

## Dependencies

- [crossterm](https://crates.io/crates/crossterm) - Terminal manipulation library
- [ratatui](https://crates.io/crates/ratatui) - Terminal UI library
- [serde](https://crates.io/crates/serde) - Serialization/deserialization framework

## License

This project is licensed under the MIT License - see the LICENSE file for details.
