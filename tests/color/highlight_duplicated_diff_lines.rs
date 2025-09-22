use annotate_snippets::{renderer::DecorStyle, AnnotationKind, Level, Patch, Renderer, Snippet};

use snapbox::{assert_data_eq, file};

#[test]
fn case() {
    // https://github.com/rust-lang/rust/blob/4b94758d2ba7d0ef71ccf5fde29ce4bc5d6fe2a4/tests/ui/argument-suggestions/wrong-highlight-span-extra-arguments-147070.rs

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
            ),
    ];

    let expected_ascii = file!["highlight_duplicated_diff_lines.ascii.term.svg": TermSvg];
    let renderer = Renderer::styled();
    assert_data_eq!(renderer.render(report), expected_ascii);

    let expected_unicode = file!["highlight_duplicated_diff_lines.unicode.term.svg": TermSvg];
    let renderer = renderer.decor_style(DecorStyle::Unicode);
    assert_data_eq!(renderer.render(report), expected_unicode);
}
