use annotate_snippets::{Message, Renderer, Slice};

fn main() {
    let message = Message::error("mismatched types")
        .slice(Slice::new("Foo", 51).origin("src/format.rs"))
        .slice(Slice::new("Faa", 129).origin("src/display.rs"));

    let renderer = Renderer::styled();
    anstream::println!("{}", renderer.render(message));
}
