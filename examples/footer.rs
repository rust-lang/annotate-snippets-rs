use annotate_snippets::{Label, Renderer, Slice, Snippet};

fn main() {
    let snippet = Snippet::error("mismatched types")
        .id("E0308")
        .slice(
            Slice::new("        slices: vec![\"A\",", 13)
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
    anstream::println!("{}", renderer.render(snippet));
}
