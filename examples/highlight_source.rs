use annotate_snippets::{AnnotationKind, Group, Level, Renderer, Snippet};

fn main() {
    let source = r#"//@ compile-flags: -Z teach

#![allow(warnings)]

const CON: Vec<i32> = vec![1, 2, 3]; //~ ERROR E0010
//~| ERROR cannot call non-const method
fn main() {}
"#;
    let message = &[Group::with_title(Level::ERROR.title("allocations are not allowed in constants")
        .id("E0010"))
        .element(
            Snippet::source(source)
                .fold(true)
                .path("$DIR/E0010-teach.rs")
                .annotation(
                    AnnotationKind::Primary
                        .span(72..85)
                        .label("allocation not allowed in constants")
                        .highlight_source(true),
                ),
        )
        .element(
            Level::NOTE.title("The runtime heap is not yet available at compile-time, so no runtime heap allocations can be created."),

    )];

    let renderer = Renderer::styled().anonymized_line_numbers(true);
    anstream::println!("{}", renderer.render(message));
}
