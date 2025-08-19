use annotate_snippets::{AnnotationKind, Group, Level, Renderer, Snippet};

fn main() {
    let message =
        &[
            Group::with_title(Level::ERROR.primary_title("mismatched types").id("E0308")).element(
                Snippet::source("        slices: vec![\"A\",")
                    .line_start(13)
                    .path("src/multislice.rs")
                    .annotation(AnnotationKind::Primary.span(21..24).label(
                        "expected struct `annotate_snippets::snippet::Slice`, found reference",
                    )),
            ),
            Group::with_title(Level::NOTE.primary_title(
                "expected type: `snippet::Annotation`\n   found type: `__&__snippet::Annotation`",
            )),
        ];

    let renderer = Renderer::styled();
    anstream::println!("{}", renderer.render(message));
}
