use annotate_snippets::{Label, Level, Renderer, Snippet};

fn main() {
    let message = Level::Error
        .title("mismatched types")
        .id("E0308")
        .snippet(
            Snippet::new("        slices: vec![\"A\",")
                .line_start(13)
                .origin("src/multislice.rs")
                .annotation(
                    Label::error(
                        "expected struct `annotate_snippets::snippet::Slice`, found reference",
                    )
                    .span(21..24),
                ),
        )
        .footer(Label::note(
            "expected type: `snippet::Annotation`\n   found type: `__&__snippet::Annotation`",
        ));

    let renderer = Renderer::styled();
    anstream::println!("{}", renderer.render(message));
}
