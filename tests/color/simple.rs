use annotate_snippets::{AnnotationKind, Group, Level, Renderer, Snippet};

use snapbox::{assert_data_eq, file};

#[test]
fn case() {
    let source = r#"        })

        for line in &self.body {
"#;

    let input = Level::ERROR
        .header("expected one of `.`, `;`, `?`, or an operator, found `for`")
        .group(
            Group::new().element(
                Snippet::source(source)
                    .path("src/format_color.rs")
                    .line_start(169)
                    .annotation(
                        AnnotationKind::Primary
                            .span(20..23)
                            .label("unexpected token"),
                    )
                    .annotation(
                        AnnotationKind::Context
                            .span(10..11)
                            .label("expected one of `.`, `;`, `?`, or an operator here"),
                    ),
            ),
        );
    let expected = file!["simple.term.svg"];
    let renderer = Renderer::styled();
    assert_data_eq!(renderer.render(input), expected);
}
