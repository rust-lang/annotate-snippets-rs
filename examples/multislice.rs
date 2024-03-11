use annotate_snippets::{Message, Renderer, Snippet};

fn main() {
    let message = Message::error("mismatched types")
        .snippet(Snippet::new("Foo").line_start(51).origin("src/format.rs"))
        .snippet(Snippet::new("Faa").line_start(129).origin("src/display.rs"));

    let renderer = Renderer::styled();
    anstream::println!("{}", renderer.render(message));
}
