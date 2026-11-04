use annotate_snippets::{Group, Level, Renderer, renderer::DecorStyle};
use snapbox::{assert_data_eq, file};

#[test]
fn case() {
    let input = &[
        Group::with_title(Level::ERROR.primary_title("title content"))
            .element(Level::ERROR.message("error message"))
            .element(Level::WARNING.message("warning message"))
            .element(Level::INFO.message("info message"))
            .element(Level::NOTE.message("note message"))
            .element(Level::HELP.message("help message")),
    ];

    let renderer = Renderer::styled();

    let expected_ascii = file!["message_labels_use_level_style.ascii.term.svg": TermSvg];
    assert_data_eq!(renderer.render(input), expected_ascii);

    let expected_unicode = file!["message_labels_use_level_style.unicode.term.svg": TermSvg];
    let renderer_unicode = renderer.decor_style(DecorStyle::Unicode);
    assert_data_eq!(renderer_unicode.render(input), expected_unicode);

    let expected_no_graphics =
        file!["message_labels_use_level_style.no_graphics.term.svg": TermSvg];
    let renderer_no_graphics = renderer_unicode.no_graphics(true);
    assert_data_eq!(renderer_no_graphics.render(input), expected_no_graphics);
}
