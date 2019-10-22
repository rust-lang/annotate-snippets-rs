#[macro_use]
extern crate criterion;

use criterion::{BatchSize, Criterion};

use annotate_snippets::*;
use std::ops::Range;

const SOURCE: &'static str = r#") -> Option<String> {
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

fn source_snippet() -> Snippet<'static, WithLineNumber<&'static str>> {
    Snippet {
        title: Some(Title {
            code: Some(&"E0308"),
            message: Message {
                text: &"mismatched types",
                level: Level::Error,
            },
        }),
        slices: &[Slice {
            span: WithLineNumber {
                line_num: 51,
                data: SOURCE,
            },
            origin: Some(&"src/format.rs"),
            annotations: &[
                Annotation {
                    span: 5..19,
                    message: Some(Message {
                        text: &"expected `Option<String>` because of return type",
                        level: Level::Warning,
                    }),
                },
                Annotation {
                    span: 26..725,
                    message: Some(Message {
                        text: &"expected enum `std::option::Option`",
                        level: Level::Error,
                    }),
                },
            ],
            footer: &[],
        }],
    }
}

fn range_snippet() -> Snippet<'static, Range<usize>> {
    Snippet {
        title: Some(Title {
            code: Some(&"E0308"),
            message: Message {
                text: &"mismatched types",
                level: Level::Error,
            },
        }),
        slices: &[Slice {
            span: 0..725,
            origin: Some(&"src/format.rs"),
            annotations: &[
                Annotation {
                    span: 5..19,
                    message: Some(Message {
                        text: &"expected `Option<String>` because of return type",
                        level: Level::Warning,
                    }),
                },
                Annotation {
                    span: 26..725,
                    message: Some(Message {
                        text: &"expected enum `std::option::Option`",
                        level: Level::Error,
                    }),
                },
            ],
            footer: &[],
        }],
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("format [&str]", |b| {
        b.iter_batched_ref(
            || Vec::<u8>::with_capacity(1100),
            |out| {
                let snippet = source_snippet();
                let formatted = format(&snippet, &());
                renderer::Ascii::new().render(&formatted, &(), out)
            },
            BatchSize::SmallInput,
        )
    });
    c.bench_function("format [Range]", |b| {
        b.iter_batched_ref(
            || Vec::<u8>::with_capacity(1100),
            |out| {
                let snippet = range_snippet();
                let formatted = format(&snippet, &SOURCE);
                renderer::Ascii::new().render(&formatted, &SOURCE, out)
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
