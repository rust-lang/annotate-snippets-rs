use annotate_snippets::{AnnotationKind, Level, Patch, Renderer, Snippet};

fn main() {
    let source = r#"fn main() {
    let _f: f32 = 0xAB_f32;
}
"#;
    let path = "$DIR/bad-hex-float-lit.rs";
    let secondary_title = "rewrite this as a decimal floating point literal, or use `as` to turn a hex literal into a float";

    let report = &[
        Level::ERROR
            .primary_title("mismatched types")
            .id("E0308")
            .element(
                Snippet::source(source)
                    .path(path)
                    .annotation(
                        AnnotationKind::Primary
                            .span(30..38)
                            .label("expected `f32`, found integer"),
                    )
                    .annotation(
                        AnnotationKind::Context
                            .span(24..27)
                            .label("expected due to this"),
                    ),
            ),
        Level::HELP
            .secondary_title(secondary_title)
            .element(
                Snippet::source(source)
                    .path(path)
                    .patch(Patch::new(30..38, "0xAB as f32")),
            )
            .element(
                Snippet::source(source)
                    .path(path)
                    .patch(Patch::new(30..38, "171_f32")),
            ),
    ];

    let renderer = Renderer::plain().no_graphics(true);
    anstream::println!("{}", renderer.render(report));
}
