use annotate_snippets::{Annotation, Group, Level, Renderer, Snippet};

fn main() {
    let message = &[Group::with_title(Level::ERROR.title("mismatched types"))
        .element(
            Snippet::<Annotation<'_>>::source("Foo")
                .line_start(51)
                .fold(false)
                .path("src/format.rs"),
        )
        .element(
            Snippet::<Annotation<'_>>::source("Faa")
                .line_start(129)
                .fold(false)
                .path("src/display.rs"),
        )];

    let renderer = Renderer::styled();
    anstream::println!("{}", renderer.render(message));
}
