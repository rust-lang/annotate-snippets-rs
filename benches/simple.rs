#![allow(clippy::unit_arg)]
#[macro_use]
extern crate criterion;

use criterion::{black_box, Criterion};

use annotate_snippets::{Label, Message, Renderer, Slice};

fn create_snippet(renderer: Renderer) {
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
    let message = Message::error("mismatched types").id("E0308").slice(
        Slice::new(source, 51)
            .origin("src/format.rs")
            .annotation(
                Label::warning("expected `Option<String>` because of return type").span(5..19),
            )
            .annotation(Label::error("expected enum `std::option::Option`").span(26..724)),
    );

    let _result = renderer.render(message).to_string();
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("format", |b| {
        b.iter(|| black_box(create_snippet(Renderer::plain())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
