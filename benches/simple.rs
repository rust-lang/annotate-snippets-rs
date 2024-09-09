use annotate_snippets::{Level, Renderer, Snippet};

#[divan::bench]
fn create_and_render() -> String {
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
    let message = Level::Error.title("mismatched types").id("E0308").snippet(
        Snippet::source(source)
            .line_start(51)
            .origin("src/format.rs")
            .annotation(
                Level::Warning
                    .span(5..19)
                    .label("expected `Option<String>` because of return type"),
            )
            .annotation(
                Level::Error
                    .span(26..724)
                    .label("expected enum `std::option::Option`"),
            ),
    );

    let renderer = Renderer::plain();
    let rendered = renderer.render(message).to_string();
    rendered
}

fn main() {
    divan::main();
}
