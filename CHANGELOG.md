# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->
## [Unreleased] - ReleaseDate

## [0.11.4] - 2024-06-15

### Fixes

- Annotations for `\r\n` are now correctly handled [#131](https://github.com/rust-lang/annotate-snippets-rs/pull/131)

## [0.11.3] - 2024-06-06

### Fixes

- Dropped MSRV to 1.65

## [0.11.2] - 2024-04-27

### Added

- All public types now implement `Debug` [#119](https://github.com/rust-lang/annotate-snippets-rs/pull/119)

## [0.11.1] - 2024-03-21

### Fixes

- Switch `fold` to use rustc's logic: always show first and last line of folded section and detect if its worth folding
- When `fold`ing the start of a `source`, don't show anything, like we do for the end of the `source`
- Render an underline for an empty span on `Annotation`s

## [0.11.0] - 2024-03-15

### Breaking Changes

- Switched from char spans to byte spans [#90](https://github.com/rust-lang/annotate-snippets-rs/pull/90/commits/b65b8cabcd34da9fed88490a7a1cd8085777706a)
- Renamed `AnnotationType` to `Level` [#94](https://github.com/rust-lang/annotate-snippets-rs/pull/94/commits/b49f9471d920c7f561fa61970039b0ba44e448ac)
- Renamed `SourceAnnotation` to `Annotation` [#94](https://github.com/rust-lang/annotate-snippets-rs/pull/94/commits/bbf9c5fe27e83652433151cbfc7d6cafc02a8c47)
- Renamed `Snippet` to `Message` [#94](https://github.com/rust-lang/annotate-snippets-rs/pull/94/commits/105da760b6e1bd4cfce4c642ac679ecf6011f511)
- Renamed `Slice` to `Snippet` [#94](https://github.com/rust-lang/annotate-snippets-rs/pull/94/commits/1c18950300cf8b93d92d89e9797ed0bae02c0a37)
- `Message`, `Snippet`, `Annotation` and `Level` can only be built with a builder pattern [#91](https://github.com/rust-lang/annotate-snippets-rs/pull/91) and [#94](https://github.com/rust-lang/annotate-snippets-rs/pull/94)
- `Annotation` labels are now optional [#94](https://github.com/rust-lang/annotate-snippets-rs/pull/94/commits/c821084068a1acd2688b6c8d0b3423e143d359e2)
- `Annotation` now takes in `Range<usize>` instead of `(usize, usize)` [#90](https://github.com/rust-lang/annotate-snippets-rs/pull/90/commits/c3bd0c3a63f983f5f2b4793a099972b1f6e97a9f)
- `Margin` is now an internal detail, only `term_width` is exposed [#105](https://github.com/rust-lang/annotate-snippets-rs/pull/105)
- `footer` was generalized to be a `Message` [#98](https://github.com/rust-lang/annotate-snippets-rs/pull/98)

### Added
- `term_width` was added to `Renderer` to control the rendering width [#105](https://github.com/rust-lang/annotate-snippets-rs/pull/105)
  - defaults to 140 when not set

### Fixed
- `Margin`s are now calculated per `Snippet`, rather than for the entire `Message` [#105](https://github.com/rust-lang/annotate-snippets-rs/pull/105)
- `Annotation`s can be created without labels

### Features
- `footer` was expanded to allow annotating sources by accepting `Message` [#98](https://github.com/rust-lang/annotate-snippets-rs/pull/98)

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
[Unreleased]: https://github.com/rust-lang/annotate-snippets-rs/compare/0.11.4...HEAD
[0.11.4]: https://github.com/rust-lang/annotate-snippets-rs/compare/0.11.3...0.11.4
[0.11.3]: https://github.com/rust-lang/annotate-snippets-rs/compare/0.11.2...0.11.3
[0.11.2]: https://github.com/rust-lang/annotate-snippets-rs/compare/0.11.1...0.11.2
[0.11.1]: https://github.com/rust-lang/annotate-snippets-rs/compare/0.11.0...0.11.1
[0.11.0]: https://github.com/rust-lang/annotate-snippets-rs/compare/0.10.2...0.11.0
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
