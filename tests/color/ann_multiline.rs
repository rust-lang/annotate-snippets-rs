use annotate_snippets::{AnnotationKind, Group, Level, Renderer, Snippet};

use snapbox::{assert_data_eq, file};

#[test]
fn case() {
    let source = r#"                        if let DisplayLine::Source {
                            ref mut inline_marks,
                        } = body[body_idx]
"#;

    let input = &[Group::with_title(
        Level::ERROR
            .primary_title("pattern does not mention fields `lineno`, `content`")
            .id("E0027"),
    )
    .element(
        Snippet::source(source)
            .path("src/display_list.rs")
            .line_start(139)
            .fold(false)
            .annotation(
                AnnotationKind::Primary
                    .span(31..128)
                    .label("missing fields `lineno`, `content`"),
            ),
    )];
    let expected = file!["ann_multiline.term.svg"];
    let renderer = Renderer::styled();
    assert_data_eq!(renderer.render(input), expected);
}
