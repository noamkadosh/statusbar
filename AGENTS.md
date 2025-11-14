# Agent Guidelines for Zellij Statusbar Plugin

## Build & Test Commands
- **Build**: `cargo build --release` (targets wasm32-wasip1 for Zellij plugin)
- **Check**: `cargo check` (fast compilation check)
- **Format**: `cargo fmt` (apply Rust formatting)
- **Lint**: `cargo clippy` (static analysis)
- **Test**: `cargo test [TESTNAME]` (run specific test by name substring)

## Code Style
- **Imports**: External crates first, then `use crate::` modules (alphabetically sorted)
- **Types**: PascalCase for structs/enums, snake_case for functions/variables
- **Modules**: One module per feature (color, datetime, layout, mode, session, tabs, view)
- **Rendering**: Use static `render()` methods on unit structs (e.g., `Mode::render()`)
- **Error Handling**: Use `eprintln!` for errors, `unwrap()` only when values are guaranteed
- **Comments**: Add inline comments for complex logic (e.g., timezone calculations, DST rules)
- **Styling**: Use `zellij_tile_utils::style!` macro for ANSI colors with palette system
- **No async**: This is a synchronous WASM plugin for Zellij

## Architecture Notes
- Plugin subscribes to Zellij events (TabUpdate, ModeUpdate, Mouse, Timer)
- All UI components return `View { blocks: Vec<Block>, len: usize }`
- Mouse interactions tracked via click position for tab switching
