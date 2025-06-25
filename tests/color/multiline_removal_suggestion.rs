use annotate_snippets::{AnnotationKind, Group, Level, Origin, Patch, Renderer, Snippet};

use snapbox::{assert_data_eq, file};

#[test]
fn case() {
    let source = r#"// Make sure suggestion for removal of a span that covers multiple lines is properly highlighted.
//@ compile-flags: --error-format=human --color=always
//@ edition:2018
//@ only-linux
// ignore-tidy-tab
// We use `\t` instead of spaces for indentation to ensure that the highlighting logic properly
// accounts for replaced characters (like we do for `\t` with `    `). The naïve way of highlighting
// could be counting chars of the original code, instead of operating on the code as it is being
// displayed.
use std::collections::{HashMap, HashSet};
fn foo() -> Vec<(bool, HashSet<u8>)> {
	let mut hm = HashMap::<bool, Vec<HashSet<u8>>>::new();
	hm.into_iter()
		.map(|(is_true, ts)| {
			ts.into_iter()
				.map(|t| {
					(
						is_true,
						t,
					)
				}).flatten()
		})
		.flatten()
		.collect()
}
fn bar() -> Vec<(bool, HashSet<u8>)> {
	let mut hm = HashMap::<bool, Vec<HashSet<u8>>>::new();
	hm.into_iter()
		.map(|(is_true, ts)| {
			ts.into_iter()
				.map(|t| (is_true, t))
				.flatten()
		})
		.flatten()
		.collect()
}
fn baz() -> Vec<(bool, HashSet<u8>)> {
	let mut hm = HashMap::<bool, Vec<HashSet<u8>>>::new();
	hm.into_iter()
		.map(|(is_true, ts)| {
			ts.into_iter().map(|t| {
				(is_true, t)
			}).flatten()
		})
		.flatten()
		.collect()
}
fn bay() -> Vec<(bool, HashSet<u8>)> {
	let mut hm = HashMap::<bool, Vec<HashSet<u8>>>::new();
	hm.into_iter()
		.map(|(is_true, ts)| {
			ts.into_iter()
				.map(|t| (is_true, t)).flatten()
		})
		.flatten()
		.collect()
}
fn main() {}
"#;

    let input = Level::ERROR
        .header("`(bool, HashSet<u8>)` is not an iterator")
        .id("E0277")
        .group(
            Group::new()
                .element(
                    Snippet::source(source)
                        .origin("$DIR/multiline-removal-suggestion.rs")
                        .fold(true)
                        .annotation(
                            AnnotationKind::Primary
                                .span(769..776)
                                .label("`(bool, HashSet<u8>)` is not an iterator"),
                        ),
                )
                .element(
                    Level::HELP
                        .title("the trait `Iterator` is not implemented for `(bool, HashSet<u8>)`"),
                )
                .element(
                    Level::NOTE
                        .title("required for `(bool, HashSet<u8>)` to implement `IntoIterator`"),
                ),
        )
        .group(
            Group::new()
                .element(Level::NOTE.title("required by a bound in `flatten`"))
                .element(
                    Origin::new("/rustc/FAKE_PREFIX/library/core/src/iter/traits/iterator.rs")
                        .line(1556)
                        .char_column(4),
                ),
        )
        .group(
            Group::new()
                .element(Level::HELP.title("consider removing this method call, as the receiver has type `std::vec::IntoIter<HashSet<u8>>` and `std::vec::IntoIter<HashSet<u8>>: Iterator` trivially holds"))
                .element(
                    Snippet::source(source)
                        .origin("$DIR/multiline-removal-suggestion.rs")
                        .fold(true)
                        .patch(Patch::new(708..768, "")),
                ),
        );
    let expected = file!["multiline_removal_suggestion.term.svg"];
    let renderer = Renderer::styled();
    assert_data_eq!(renderer.render(input), expected);
}
