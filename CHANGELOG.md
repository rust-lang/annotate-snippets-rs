# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->
## [Unreleased] - ReleaseDate


## [0.10.2] - 2024-02-29

### Added

- Added `testing-colors` feature to remove platform-specific colors when testing
  [#82](https://github.com/rust-lang/annotate-snippets-rs/pull/82)

## [0.10.1] - 2024-01-04

### Fixed

- Match `rustc`'s colors [#73](https://github.com/rust-lang/annotate-snippets-rs/pull/73)
- Allow highlighting one past the end of `source` [#74](https://github.com/rust-lang/annotate-snippets-rs/pull/74)

### Compatibility

- Set the minimum supported Rust version to `1.73.0` [#71](https://github.com/rust-lang/annotate-snippets-rs/pull/71)

## [0.10.0] - December 12, 2023

### Added

- `Renderer` is now used for displaying a `Snippet` [#67](https://github.com/rust-lang/annotate-snippets-rs/pull/67/commits/9076cbf66336e5137b47dc7a52df2999b6c82598)
  - `Renderer` also controls the color scheme and formatting of the snippet

### Changed

- Moved everything in the `snippet` to be in the crate root [#67](https://github.com/rust-lang/annotate-snippets-rs/pull/67/commits/a1007ddf2fc6f76e960a4fc01207228e64e9fae7)

### Breaking Changes

- `Renderer` now controls the color scheme and formatting of `Snippet`s [#67](https://github.com/rust-lang/annotate-snippets-rs/pull/67/commits/d0c65b26493d60f86a82c5919ef736b35808c23a)
- Removed the `Style` and `Stylesheet` traits, as color is controlled by `Renderer` [#67](https://github.com/rust-lang/annotate-snippets-rs/pull/67/commits/4affdfb50ea0670d85e52737c082c03f89ae8ada)
- Replaced [`yansi-term`](https://crates.io/crates/yansi-term) with [`anstyle`](https://crates.io/crates/anstyle) [#67](https://github.com/rust-lang/annotate-snippets-rs/pull/67/commits/dfd4e87d6f31ec50d29af26d7310cff5e66ca978)
  - `anstyle` is designed primarily to exist in public APIs for interoperability 
  - `anstyle` is re-exported under `annotate_snippets::renderer`
- Removed the `color` feature in favor of `Renderer::plain()` [#67](https://github.com/rust-lang/annotate-snippets-rs/pull/67/commits/dfd4e87d6f31ec50d29af26d7310cff5e66ca978)
- Moved `Margin` to `renderer` module [#67](https://github.com/rust-lang/annotate-snippets-rs/pull/67/commits/79f657ea252c3c0ce55fa69894ee520f8820b4bf)
- Made the `display_list` module private [#67](https://github.com/rust-lang/annotate-snippets-rs/pull/67/commits/da45f4858af3ec4c0d792ecc40225e27fdd2bac8)

### Compatibility

- Changed the edition to `2021` [#61](https://github.com/rust-lang/annotate-snippets-rs/pull/61)
- Set the minimum supported Rust version to `1.70.0` [#61](https://github.com/rust-lang/annotate-snippets-rs/pull/61)

## [0.9.2] - October 30, 2023

- Remove parsing of __ in title strings, fixes (#53)
- Origin line number is not correct when using a slice with fold: true (#52)

## [0.9.1] - September 4, 2021

- Fix character split when strip code. (#37)
- Fix off by one error in multiline highlighting. (#42)
- Fix display of annotation for double width characters. (#46)

## [0.9.0] - June 28, 2020

- Add strip code to the left and right of long lines. (#36)

## [0.8.0] - April 14, 2020

- Replace `ansi_term` with `yansi-term` for improved performance. (#30)
- Turn `Snippet` and `Slice` to work on borrowed slices, rather than Strings. (#32)
- Fix `\r\n` end of lines. (#29)

## [0.7.0] - March 30, 2020

- Refactor API to use `fmt::Display` (#27)
- Fix SourceAnnotation range (#27)
- Fix column numbers (#22)
- Derive `PartialEq` for `AnnotationType` (#19)
- Update `ansi_term` to 0.12.

## [0.6.1] - July 23, 2019

- Fix too many anonymized line numbers (#5)
 
## [0.6.0] - June 26, 2019
 
- Add an option to anonymize line numbers (#3)
- Transition the crate to rust-lang org.
- Update the syntax to Rust 2018 idioms. (#4)

<!-- next-url -->
[Unreleased]: https://github.com/rust-lang/annotate-snippets-rs/compare/0.10.2...HEAD
[0.10.2]: https://github.com/rust-lang/annotate-snippets-rs/compare/0.10.1...0.10.2
[0.10.1]: https://github.com/rust-lang/annotate-snippets-rs/compare/0.10.0...0.10.1
[0.10.0]: https://github.com/rust-lang/annotate-snippets-rs/compare/0.9.2...0.10.0
[0.9.2]: https://github.com/rust-lang/annotate-snippets-rs/compare/0.9.1...0.9.2
[0.9.1]: https://github.com/rust-lang/annotate-snippets-rs/compare/0.9.0...0.9.1
[0.9.0]: https://github.com/rust-lang/annotate-snippets-rs/compare/0.8.0...0.9.0
[0.8.0]: https://github.com/rust-lang/annotate-snippets-rs/compare/0.7.0...0.8.0
[0.7.0]: https://github.com/rust-lang/annotate-snippets-rs/compare/0.6.1...0.7.0
[0.6.1]: https://github.com/rust-lang/annotate-snippets-rs/compare/0.6.0...0.6.1
[0.6.0]: https://github.com/rust-lang/annotate-snippets-rs/compare/0.5.0...0.6.0
[0.5.0]: https://github.com/rust-lang/annotate-snippets-rs/compare/0.1.0...0.5.0
[0.1.0]: https://github.com/rust-lang/annotate-snippets-rs/compare/6015d08d7d10151c126c6a70c14f234c0c01b50e...0.1.0
