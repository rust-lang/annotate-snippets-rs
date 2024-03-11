use annotate_snippets::{Message, Renderer, Snippet};

fn main() {
    let message = Message::error("mismatched types")
        .snippet(Snippet::new("Foo", 51).origin("src/format.rs"))
        .snippet(Snippet::new("Faa", 129).origin("src/display.rs"));

    let renderer = Renderer::styled();
    anstream::println!("{}", renderer.render(message));
}
