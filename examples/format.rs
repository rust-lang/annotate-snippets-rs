use annotate_snippets::slice::{AnnotationType, SourceAnnotation};
use annotate_snippets::Slice;

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

    let slice = Slice {
        source,
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
                range: (23, 704),
            },
        ],
    };
    println!("{}", slice);
}
