use annotate_snippets::DisplayList;
use annotate_snippets::{Annotation, AnnotationType, InlineAnnotation, SourceAnnotation};
use annotate_snippets::{Slice, Snippet};

use annotate_snippets::renderers::get_renderer;
use annotate_snippets::renderers::Renderer;

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

    let snippet = Snippet {
        title: Some(Annotation {
            id: Some("E0308"),
            label: Some("mismatched types"),
            annotation_type: AnnotationType::Error,
        }),
        footer: &[],
        slices: &[Slice {
            source,
            line_start: Some(51),
            origin: Some("src/format.rs"),
            annotations: &[
                SourceAnnotation {
                    label: "expected `Option<String>` because of return type",
                    annotation_type: AnnotationType::Warning,
                    range: 5..19,
                },
                SourceAnnotation {
                    label: "expected enum `std::option::Option`",
                    annotation_type: AnnotationType::Error,
                    range: 23..725,
                },
            ],
            inline_annotations: &[
                InlineAnnotation {
                    annotation_type: AnnotationType::Warning,
                    range: 5..19,
                },
                InlineAnnotation {
                    annotation_type: AnnotationType::Error,
                    range: 49..50,
                },
                InlineAnnotation {
                    annotation_type: AnnotationType::Error,
                    range: 724..725,
                },
                InlineAnnotation {
                    annotation_type: AnnotationType::Help,
                    range: 421..427,
                },
            ],
        }],
    };
    let dl = DisplayList::from(&snippet);
    let r = get_renderer();

    let mut s: Vec<u8> = Vec::new();
    r.fmt(&mut s, &dl).unwrap();
    println!("{}", std::str::from_utf8(&s).unwrap());
}
