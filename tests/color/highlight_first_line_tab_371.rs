use annotate_snippets::{Level, Patch, Renderer, Snippet};

use snapbox::{assert_data_eq, file};

#[test]
fn test() {
    let report = &[Level::ERROR
        .primary_title("<sample error message>")
        .element(
            Snippet::source(
                "\t\t\tts.into_iter().map(|t| {\n\t\t\t\t(is_true, t)\n\t\t\t}).flatten()\n",
            )
            .line_start(6)
            .path("<sample path>")
            .patch(Patch::new(17..50, "")),
        )];

    let expected_ascii = file!["highlight_first_line_tab_371.ascii.term.svg": TermSvg];
    let renderer = Renderer::styled();
    assert_data_eq!(renderer.render(report), expected_ascii);

    let expected_no_graphics = file!["highlight_first_line_tab_371.no_graphics.term.svg": TermSvg];
    let renderer = renderer.no_graphics(true);
    assert_data_eq!(renderer.render(report), expected_no_graphics);
}
