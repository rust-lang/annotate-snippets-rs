use annotate_snippets::{level::Level, Annotation, Group, Renderer, Snippet};

fn main() {
    let message = Level::ERROR.message("mismatched types").group(
        Group::new()
            .element(
                Snippet::<Annotation<'_>>::source("Foo")
                    .line_start(51)
                    .origin("src/format.rs"),
            )
            .element(
                Snippet::<Annotation<'_>>::source("Faa")
                    .line_start(129)
                    .origin("src/display.rs"),
            ),
    );

    let renderer = Renderer::styled();
    anstream::println!("{}", renderer.render(message));
}
