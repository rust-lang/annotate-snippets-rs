# annotate-snippets

`annotate-snippets` is a Rust library designed to annotate programming code slices, making debugging and diagnostic rendering more intuitive and visually appealing.

[![crates.io](https://img.shields.io/crates/v/annotate-snippets.svg)](https://crates.io/crates/annotate-snippets)
[![documentation](https://img.shields.io/badge/docs-master-blue.svg)](https://docs.rs/annotate-snippets/)
![build status](https://github.com/rust-lang/annotate-snippets-rs/actions/workflows/ci.yml/badge.svg)

About the Library
-----------------
The annotate-snippets library provides tools to annotate source code slices with rich, meta information. It generates visually organized, styled error or informational messages, helping developers quickly pinpoint issues in their code.

Key Feature:
------------
It takes a data structure called `Snippet` on the input and produces a `String`
which may look like this:

![Screenshot](./examples/expected_type.svg)

Local Development
-----------------
To build and test the project locally, use the following commands:
    
    cargo build
    cargo test

Formatting
----------
Before submitting a pull request (PR), ensure the code follows Rust’s formatting standards by running:
    
    cargo fmt

Contributing
------------
When submitting a PR please use  [`cargo fmt`](https://github.com/rust-lang/rustfmt)  
**NOTE:-** `cargo fmt` requires the nightly version of Rust.

Core Concepts:
--------------
`Annotation`: Adding labels or notes to specific parts of code slices for clarity.

`Snippet`: A small section of programming code to which annotations and diagnostics are applied.

`Diagnostic Rendering`: Presenting errors or meta information visually for better debugging.

`Renderer`: A mechanism that transforms the annotated snippets into styled, readable output.

`Meta Information`: Data that gives additional context or details about a code snippet, like where an error occurred or what caused it.

`Feature Parity`: Ensuring two tools (or versions of the same tool) have the same capabilities

`Error Severity Levels`: Categories that describe how bad an error is (e.g., Error, Warning, Info) to help prioritize fixes.

`Programming code slices`: are small, specific portions or snippets of a larger codebase.
