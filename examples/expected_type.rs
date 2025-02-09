use annotate_snippets::{AnnotationKind, Group, Level, Renderer, Snippet};

fn main() {
    let source = r#"                annotations: vec![SourceAnnotation {
                label: "expected struct `annotate_snippets::snippet::Slice`, found reference"
                    ,
                range: <22, 25>,"#;
    let message =
        Level::Error.message("expected type, found `22`").group(
            Group::new().element(
                Snippet::source(source)
                    .line_start(26)
                    .origin("examples/footer.rs")
                    .fold(true)
                    .annotation(AnnotationKind::Primary.span(193..195).label(
                        "expected struct `annotate_snippets::snippet::Slice`, found reference",
                    ))
                    .annotation(
                        AnnotationKind::Context
                            .span(34..50)
                            .label("while parsing this struct"),
                    ),
            ),
        );

    let renderer = Renderer::styled();
    anstream::println!("{}", renderer.render(message));
}
