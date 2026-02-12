use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet, renderer::DecorStyle};
use snapbox::{assert_data_eq, file};

#[test]
fn simple() {
    // Most basic test: check that the a hyperlink shows up in the error message displayed in the
    // README.
    let source = r#"                annotations: vec![SourceAnnotation {
                label: "expected struct `annotate_snippets::snippet::Slice`, found reference"
                    ,
                range: <22, 25>,"#;
    let report =
        &[Level::ERROR
            .primary_title("expected type, found `22`")
            .element(
                Snippet::source(source)
                    .line_start(26)
                    .path("examples/footer.rs")
                    .path_url("file://localhost/home/user/rust/file.rs")
                    .annotation(AnnotationKind::Primary.span(193..195).label(
                        "expected struct `annotate_snippets::snippet::Slice`, found reference",
                    ))
                    .annotation(
                        AnnotationKind::Context
                            .span(34..50)
                            .label("while parsing this struct"),
                    ),
            )];

    let expected_ascii = file!["hyperlink_expected_type.ascii.term.svg": TermSvg];
    let renderer = Renderer::styled().decor_style(DecorStyle::Ascii);
    assert_data_eq!(renderer.render(report), expected_ascii);

    let expected_unicode = file!["hyperlink_expected_type.unicode.term.svg": TermSvg];
    let renderer = Renderer::styled().decor_style(DecorStyle::Unicode);
    assert_data_eq!(renderer.render(report), expected_unicode);
}
