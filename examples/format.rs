use annotate_snippets::{level::Level, AnnotationKind, Group, Renderer, Snippet};

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
    let message = Level::ERROR.message("mismatched types").id("E0308").group(
        Group::new().element(
            Snippet::source(source)
                .line_start(51)
                .origin("src/format.rs")
                .annotation(
                    AnnotationKind::Context
                        .span(5..19)
                        .label("expected `Option<String>` because of return type"),
                )
                .annotation(
                    AnnotationKind::Primary
                        .span(26..724)
                        .label("expected enum `std::option::Option`"),
                ),
        ),
    );

    let renderer = Renderer::styled();
    anstream::println!("{}", renderer.render(message));
}
