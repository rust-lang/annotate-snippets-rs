use annotate_snippets::{Label, Message, Renderer, Slice};

fn main() {
    let source = r#"                annotations: vec![SourceAnnotation {
                label: "expected struct `annotate_snippets::snippet::Slice`, found reference"
                    ,
                range: <22, 25>,"#;
    let message = Message::error("expected type, found `22`").slice(
        Slice::new(source, 26)
            .origin("examples/footer.rs")
            .fold(true)
            .annotation(
                Label::error(
                    "expected struct `annotate_snippets::snippet::Slice`, found reference",
                )
                .span(193..195),
            )
            .annotation(Label::info("while parsing this struct").span(34..50)),
    );

    let renderer = Renderer::styled();
    anstream::println!("{}", renderer.render(message));
}
