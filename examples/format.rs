use annotate_snippets::{Label, Renderer, Slice, Snippet};

fn main() {
    let source = r#") -> Option<String> {
    for ann in annotations {
        match (ann.range.0, ann.range.1) {
            (None, None) => continue,
            (Some(start), Some(end)) if start > end_index => continue,
            (Some(start), Some(end)) if start >= start_index => {
                let label = if let Some(ref label) = ann.label {
                    format!(" {}", label)
                } else {
                    String::from("")
                };

                return Some(format!(
                    "{}{}{}",
                    " ".repeat(start - start_index),
                    "^".repeat(end - start),
                    label
                ));
            }
            _ => continue,
        }
    }"#;
    let snippet = Snippet::error("mismatched types").id("E0308").slice(
        Slice::new(source, 51)
            .origin("src/format.rs")
            .annotation(
                Label::warning("expected `Option<String>` because of return type").span(5..19),
            )
            .annotation(Label::error("expected enum `std::option::Option`").span(26..724)),
    );

    let renderer = Renderer::plain();
    println!("{}", renderer.render(snippet));
}
