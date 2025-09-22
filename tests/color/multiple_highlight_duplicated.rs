use annotate_snippets::{renderer::DecorStyle, AnnotationKind, Level, Patch, Renderer, Snippet};

use snapbox::{assert_data_eq, file};

#[test]
fn case() {
    let source = r#"struct Thingie;

impl Thingie {
    pub(crate) fn new(
        _a: String,
        _b: String,
        _c: String,
        _d: String,
        _e: String,
        _f: String,
    ) -> Self {
        unimplemented!()
    }
}

fn main() {
    let foo = Thingie::new(
        String::from(""),
        String::from(""),
        String::from(""),
        String::from(""),
        String::from(""),
        String::from(""),
        String::from(""),
    );
}"#;

    let path = "$DIR/wrong-highlight-span-extra-arguments-147070.rs";

    let report = &[
        Level::ERROR
            .primary_title("this function takes 6 arguments but 7 arguments were supplied")
            .id("E0061")
            .element(
                Snippet::source(source)
                    .path(path)
                    .annotation(
                        AnnotationKind::Context
                            .span(429..445)
                            .label("unexpected argument #7 of type `String`"),
                    )
                    .annotation(AnnotationKind::Primary.span(251..263)),
            ),
        Level::NOTE
            .secondary_title("associated function defined here")
            .element(
                Snippet::source(source)
                    .path(path)
                    .annotation(AnnotationKind::Primary.span(50..53)),
            ),
        Level::HELP
            .secondary_title("remove the extra argument")
            .element(
                Snippet::source(source)
                    .path(path)
                    .patch(Patch::new(419..445, "")),
            )
            .element(
                Snippet::source(source)
                    .path(path)
                    .patch(Patch::new(266..292, "")),
            )
            .element(
                Snippet::source(source)
                    .path(path)
                    .patch(Patch::new(289..315, "")),
            )
            .element(
                Snippet::source(source)
                    .path(path)
                    .patch(Patch::new(403..420, "")),
            )
            .element(
                Snippet::source(source)
                    .path(path)
                    .patch(Patch::new(419..445, "")),
            ),
    ];

    let expected_ascii = file!["multiple_highlight_duplicated.ascii.term.svg": TermSvg];
    let renderer = Renderer::styled();
    assert_data_eq!(renderer.render(report), expected_ascii);

    let expected_unicode = file!["multiple_highlight_duplicated.unicode.term.svg": TermSvg];
    let renderer = renderer.decor_style(DecorStyle::Unicode);
    assert_data_eq!(renderer.render(report), expected_unicode);
}
