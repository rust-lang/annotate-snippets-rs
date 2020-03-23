#![allow(clippy::unit_arg)]
#[macro_use]
extern crate criterion;

use criterion::{black_box, Criterion};

use annotate_snippets::{
    display_list::{DisplayList, FormatOptions},
    snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation},
};

fn create_snippet() {
    let snippet = Snippet {
        slices: vec![Slice {
            source: r#") -> Option<String> {
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
    }"#
            .to_string(),
            line_start: 51,
            origin: Some("src/format.rs".to_string()),
            fold: false,
            annotations: vec![
                SourceAnnotation {
                    label: "expected `Option<String>` because of return type".to_string(),
                    annotation_type: AnnotationType::Warning,
                    range: (5, 19),
                },
                SourceAnnotation {
                    label: "expected enum `std::option::Option`".to_string(),
                    annotation_type: AnnotationType::Error,
                    range: (23, 745),
                },
            ],
        }],
        title: Some(Annotation {
            label: Some("mismatched types".to_string()),
            id: Some("E0308".to_string()),
            annotation_type: AnnotationType::Error,
        }),
        footer: vec![],
        opt: FormatOptions {
            color: true,
            anonymized_line_numbers: false,
        },
    };

    let dl = DisplayList::from(snippet);
    let _result = dl.to_string();
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("format", |b| b.iter(|| black_box(create_snippet())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
