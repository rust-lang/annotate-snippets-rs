use annotate_snippets::{renderer::DecorStyle, AnnotationKind, Group, Level, Renderer, Snippet};

use snapbox::{assert_data_eq, file};

#[test]
fn case() {
    let input = &[
        Group::with_title(Level::ERROR.primary_title("expected `.`, `=`")).element(
            Snippet::source("asdf")
                .path("Cargo.toml")
                .line_start(1)
                .annotation(AnnotationKind::Primary.span(4..4).label("")),
        ),
    ];

    let expected_ascii = file!["ann_eof.ascii.term.svg": TermSvg];
    let renderer = Renderer::styled();
    assert_data_eq!(renderer.render(input), expected_ascii);

    let expected_unicode = file!["ann_eof.unicode.term.svg": TermSvg];
    let renderer = renderer.decor_style(DecorStyle::Unicode);
    assert_data_eq!(renderer.render(input), expected_unicode);
}
