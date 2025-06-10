use annotate_snippets::renderer::OutputTheme;
use annotate_snippets::{AnnotationKind, Group, Level, Renderer, Snippet};

fn main() {
    let source = r#"//@ compile-flags: -Zterminal-urls=yes
fn main() {
    let () = 4; //~ ERROR
}
"#;

    let message = Level::ERROR.header("mismatched types").id("E0308").group(
        Group::new().element(
            Snippet::source(source)
                .line_start(1)
                .path("$DIR/terminal_urls.rs")
                .fold(true)
                .annotation(
                    AnnotationKind::Primary
                        .span(59..61)
                        .label("expected integer, found `()`"),
                )
                .annotation(
                    AnnotationKind::Context
                        .span(64..65)
                        .label("this expression has type `{integer}`"),
                ),
        ),
    );

    let renderer = Renderer::styled().theme(OutputTheme::Unicode);
    anstream::println!("{}", renderer.render(message));
}
