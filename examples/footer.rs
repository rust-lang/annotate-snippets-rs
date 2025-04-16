use annotate_snippets::{level::Level, AnnotationKind, Group, Renderer, Snippet};

fn main() {
    let message = Level::ERROR
        .message("mismatched types")
        .id("E0308")
        .group(
            Group::new().element(
                Snippet::source("        slices: vec![\"A\",")
                    .line_start(13)
                    .origin("src/multislice.rs")
                    .annotation(AnnotationKind::Primary.span(21..24).label(
                        "expected struct `annotate_snippets::snippet::Slice`, found reference",
                    )),
            ),
        )
        .group(Group::new().element(Level::NOTE.title(
            "expected type: `snippet::Annotation`\n   found type: `__&__snippet::Annotation`",
        )));

    let renderer = Renderer::styled();
    anstream::println!("{}", renderer.render(message));
}
