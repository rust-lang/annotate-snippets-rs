# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased] - ReleaseDate
 
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
[Unreleased]: https://github.com/rust-lang/annotate-snippets-rs/compare/0.9.2...HEAD
[0.9.2]: https://github.com/rust-lang/annotate-snippets-rs/compare/0.9.1...0.9.2
[0.9.1]: https://github.com/rust-lang/annotate-snippets-rs/compare/0.9.0...0.9.1
[0.9.0]: https://github.com/rust-lang/annotate-snippets-rs/compare/0.8.0...0.9.0
[0.8.0]: https://github.com/rust-lang/annotate-snippets-rs/compare/0.7.0...0.8.0
[0.7.0]: https://github.com/rust-lang/annotate-snippets-rs/compare/0.6.1...0.7.0
[0.6.1]: https://github.com/rust-lang/annotate-snippets-rs/compare/0.6.0...0.6.1
[0.6.0]: https://github.com/rust-lang/annotate-snippets-rs/compare/0.5.0...0.6.0
[0.5.0]: https://github.com/rust-lang/annotate-snippets-rs/compare/0.1.0...0.5.0
[0.1.0]: https://github.com/rust-lang/annotate-snippets-rs/compare/6015d08d7d10151c126c6a70c14f234c0c01b50e...0.1.0
