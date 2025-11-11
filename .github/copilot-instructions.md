# AI Agent Instructions for rust-learning

This repository contains a Rust learning path with multiple small projects demonstrating Rust programming concepts. Here's what you need to know to assist effectively:

## Project Structure

- **Root Projects**: Each concept has its own directory at the root level
  - `hello-world/`: Basic Rust program compilation example
  - `primitive-data-type/`: Examples of Rust's primitive types
  - `fvw-workspace/`: Cargo-based project example

## Build Patterns

Two main compilation approaches are used:

1. **Direct rustc compilation** (for single-file examples):
   ```bash
   rustc filename.rs
   ./filename
   ```
   Used in: `hello-world/`, `primitive-data-type/`

2. **Cargo-based builds** (for workspace projects):
   ```bash
   cargo run
   ```
   Used in: `fvw-workspace/`

## Development Workflow

1. Navigate to the specific concept directory
2. For single files: Use `rustc` to compile and run the binary
3. For Cargo projects: Use `cargo run` to build and execute

## Dependencies

Currently minimal external dependencies. Projects focus on Rust standard library features.