use annotate_snippets::{AnnotationKind, Group, Level, Renderer, Snippet};

use snapbox::{assert_data_eq, file};

#[test]
fn case() {
    let source = r#"

invalid syntax
"#;

    let input = Level::ERROR.header("").group(
        Group::new().element(
            Snippet::source(source)
                .path("path/to/error.rs")
                .line_start(1)
                .fold(true)
                .annotation(AnnotationKind::Context.span(2..16).label("error here")),
        ),
    );
    let expected = file!["fold_bad_origin_line.term.svg"];
    let renderer = Renderer::styled();
    assert_data_eq!(renderer.render(input), expected);
}
