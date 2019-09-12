#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;

use std::fmt::Write;

use annotate_snippets::{Annotation, AnnotationType, SourceAnnotation};
use annotate_snippets::{Slice, Snippet};
use annotate_snippets::DisplayList;

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

fn create_snippet() {
    let snippet = Snippet {
        title: Some(Annotation {
            id: Some("E0308"),
            label: Some("mismatched types"),
            annotation_type: AnnotationType::Error,
        }),
        footer: &[],
        slices: &[Slice {
            source: SOURCE,
            line_start: Some(51),
            origin: Some("src/format.rs"),
            annotations: vec![
                SourceAnnotation {
                    label: "expected `Option<String>` because of return type",
                    annotation_type: AnnotationType::Warning,
                    range: (5, 19),
                },
                SourceAnnotation {
                    label: "expected enum `std::option::Option`",
                    annotation_type: AnnotationType::Error,
                    range: (23, 725),
                },
            ],
        }],
    };
    let dl: DisplayList = (&snippet).into();
    let mut result = String::new();
    write!(result, "{}", dl).unwrap();
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("format", |b| b.iter(|| black_box(create_snippet())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
