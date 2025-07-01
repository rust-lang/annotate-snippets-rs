use annotate_snippets::{AnnotationKind, Group, Level, Renderer, Snippet};

use snapbox::{assert_data_eq, file};

#[test]
fn case() {
    let source = r#"lints = 20

[workspace]

[package]
name = "hello"
version = "1.0.0"
license = "MIT"
rust-version = "1.70"
edition = "2021"
"#;

    let input = &[Group::with_title(
        Level::ERROR
            .title("invalid type: integer `20`, expected a lints table")
            .id("E0308"),
    )
    .element(
        Snippet::source(source)
            .path("Cargo.toml")
            .line_start(1)
            .fold(true)
            .annotation(AnnotationKind::Primary.span(8..10).label("")),
    )];
    let expected = file!["fold_trailing.term.svg"];
    let renderer = Renderer::styled();
    assert_data_eq!(renderer.render(input), expected);
}
