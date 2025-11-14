# Zellij Statusbar Plugin

A custom statusbar plugin for [Zellij](https://zellij.dev/) terminal workspace, written in Rust and compiled to WebAssembly.

## Overview

This plugin provides a feature-rich statusbar that displays essential information about your Zellij session, including the current mode, active layout, tabs, session name, and current time. It includes full mouse support for intuitive tab navigation.

## Features

- **Mode Indicator**: Shows the current Zellij input mode (Normal, Pane, Tab, Resize, Scroll, etc.) with color-coded styling
- **Layout Display**: Displays the active pane layout configuration
- **Tab Management**: 
  - Visual list of all tabs with the active tab highlighted
  - Click on any tab to switch to it
  - Scroll wheel support to navigate between tabs
  - Sync panes indicator when enabled
  - Rename mode support with visual feedback
- **Session Information**: Displays the current session name
- **Date & Time**: Real-time clock updated every second
- **Adaptive Layout**: Automatically adjusts to terminal width with graceful overflow handling
- **Theme Support**: Respects Zellij's theme colors (dark/light mode)

## Building

Run the following command:

```shell
cargo build --release
```

## Installing

Copy the file `target/wasm32-wasi/release/statusbar.wasm` to the Zellij plugin config directory `~/.config/zellij/plugins`.
