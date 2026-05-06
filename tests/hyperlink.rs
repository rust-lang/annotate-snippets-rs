use annotate_snippets::{AnnotationKind, Level, Patch, Renderer, Snippet, renderer::DecorStyle};
use snapbox::{assert_data_eq, file};

const SOURCE: &str = r#"                annotations: vec![SourceAnnotation {
                label: "expected struct `annotate_snippets::snippet::Slice`, found reference"
                    ,
                range: <22, 25>,"#;

#[test]
fn hardcoded() {
    let report =
        &[Level::ERROR
            .primary_title("expected type, found `22`")
            .element(
                Snippet::source(SOURCE)
                    .line_start(26)
                    .path("footer.rs")
                    .path_url("file://sample_hostname/home/user/footer.rs:28")
                    .annotation(AnnotationKind::Primary.span(193..195).label(
                        "expected struct `annotate_snippets::snippet::Slice`, found reference",
                    ))
                    .annotation(
                        AnnotationKind::Context
                            .span(34..50)
                            .label("while parsing this struct"),
                    ),
            )];

    let expected = file!["hyperlink_hardcoded.ascii.term.svg": TermSvg];
    let renderer = Renderer::styled().decor_style(DecorStyle::Ascii);
    assert_data_eq!(renderer.render(report), expected);
}

#[test]
fn with_formatter() {
    let report = &[Level::ERROR
        .primary_title("expected expression, found `<`")
        .element(
            Snippet::source(SOURCE)
                .line_start(26)
                .path("bad_expression.rs")
                .path_url(|line, _col| format!("file://sample_hostname/home/user/footer.rs:{line}"))
                .annotation(
                    AnnotationKind::Primary
                        .span(192..193)
                        .label("expected expression, found `<`"),
                ),
        )];

    let expected = file!["hyperlink_with_formatter.ascii.term.svg": TermSvg];
    let renderer = Renderer::styled().decor_style(DecorStyle::Ascii);
    assert_data_eq!(renderer.render(report), expected);
}

#[test]
fn in_patch_origin() {
    let report = &[Level::ERROR
        .primary_title("<sample error message>")
        .element(
            Snippet::source(
                "\t\t\tts.into_iter().map(|t| {\n\t\t\t\t(is_true, t)\n\t\t\t}).flatten()\n",
            )
            .line_start(6)
            .path("issue_371.rs")
            .path_url("file://sample_hostname/home/user/issue_371.rs:6")
            .patch(Patch::new(17..50, "")),
        )];

    let expected_ascii = file!["hyperlink_in_patch_origin.ascii.term.svg": TermSvg];
    let renderer = Renderer::styled();
    assert_data_eq!(renderer.render(report), expected_ascii);
}
