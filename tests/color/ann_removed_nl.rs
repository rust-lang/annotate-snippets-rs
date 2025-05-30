use annotate_snippets::{AnnotationKind, Group, Level, Renderer, Snippet};

use snapbox::{assert_data_eq, file};

#[test]
fn case() {
    let input = Level::ERROR.header("expected `.`, `=`").group(
        Group::new().element(
            Snippet::source("asdf")
                .origin("Cargo.toml")
                .line_start(1)
                .annotation(AnnotationKind::Primary.span(4..5).label("")),
        ),
    );
    let expected = file!["ann_removed_nl.term.svg"];
    let renderer = Renderer::styled();
    assert_data_eq!(renderer.render(input), expected);
}
