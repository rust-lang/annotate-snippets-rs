use annotate_snippets::{level::Level, AnnotationKind, Group, Renderer, Snippet};

#[divan::bench]
fn simple() -> String {
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

    let renderer = Renderer::plain();
    let rendered = renderer.render(message);
    rendered
}

#[divan::bench(args=[0, 1, 10, 100, 1_000, 10_000, 100_000])]
fn fold(bencher: divan::Bencher<'_, '_>, context: usize) {
    bencher
        .with_inputs(|| {
            let line = "012345678901234567890123456789";
            let mut input = String::new();
            for _ in 1..=context {
                input.push_str(line);
                input.push('\n');
            }
            let span_start = input.len() + line.len();
            let span = span_start..span_start;

            input.push_str(line);
            input.push('\n');
            for _ in 1..=context {
                input.push_str(line);
                input.push('\n');
            }
            (input, span)
        })
        .bench_values(|(input, span)| {
            let message = Level::ERROR.message("mismatched types").id("E0308").group(
                Group::new().element(
                    Snippet::source(&input)
                        .fold(true)
                        .origin("src/format.rs")
                        .annotation(
                            AnnotationKind::Context
                                .span(span)
                                .label("expected `Option<String>` because of return type"),
                        ),
                ),
            );

            let renderer = Renderer::plain();
            let rendered = renderer.render(message);
            rendered
        });
}

fn main() {
    divan::main();
}
