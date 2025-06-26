use annotate_snippets::{AnnotationKind, Group, Level, Renderer, Snippet};

use snapbox::{assert_data_eq, file};

#[test]
fn case() {
    let source = r#"This is an example
of an edge case of an annotation overflowing
to exactly one character on next line.
"#;

    let input = Level::ERROR
        .header("spacing error found")
        .id("E####")
        .group(
            Group::new().element(
                Snippet::source(source)
                    .path("foo.txt")
                    .line_start(26)
                    .fold(false)
                    .annotation(
                        AnnotationKind::Primary
                            .span(11..19)
                            .label("this should not be on separate lines"),
                    ),
            ),
        );
    let expected = file!["ann_multiline2.term.svg"];
    let renderer = Renderer::styled();
    assert_data_eq!(renderer.render(input), expected);
}
