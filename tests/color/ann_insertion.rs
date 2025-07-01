use annotate_snippets::{AnnotationKind, Group, Level, Renderer, Snippet};

use snapbox::{assert_data_eq, file};

#[test]
fn case() {
    let input = &[
        Group::with_title(Level::ERROR.title("expected `.`, `=`")).element(
            Snippet::source("asf")
                .path("Cargo.toml")
                .line_start(1)
                .annotation(AnnotationKind::Primary.span(2..2).label("'d' belongs here")),
        ),
    ];
    let expected = file!["ann_insertion.term.svg"];
    let renderer = Renderer::styled();
    assert_data_eq!(renderer.render(input), expected);
}
