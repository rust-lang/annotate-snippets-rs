use annotate_snippets::{AnnotationKind, Group, Level, Renderer, Snippet};

use snapbox::{assert_data_eq, file};

#[test]
fn case() {
    let input = &[Group::new()
        .element(Level::ERROR.title("expected `.`, `=`"))
        .element(
            Snippet::source("asdf")
                .path("Cargo.toml")
                .line_start(1)
                .annotation(AnnotationKind::Primary.span(4..4).label("")),
        )];
    let expected = file!["ann_eof.term.svg"];
    let renderer = Renderer::styled();
    assert_data_eq!(renderer.render(input), expected);
}
