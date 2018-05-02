extern crate annotate_snippets;

use annotate_snippets::display_list::DisplayList;
use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation};

fn main() {
    let snippet = Snippet {
        title: Some(Annotation {
            label: Some("mismatched types".to_string()),
            id: Some("E0308".to_string()),
            annotation_type: AnnotationType::Error,
        }),
        footer: Some(Annotation {
            label: Some(
                "expected type: `snippet::Annotation`\n   found type: `__&__snippet::Annotation`"
                    .to_string(),
            ),
            id: None,
            annotation_type: AnnotationType::Note,
        }),
        slices: vec![Slice {
            source: "        slices: vec![\"A\",".to_string(),
            line_start: 13,
            origin: Some("src/multislice.rs".to_string()),
            fold: false,
            annotations: vec![SourceAnnotation {
                label: "expected struct `annotate_snippets::snippet::Slice`, found reference"
                    .to_string(),
                annotation_type: AnnotationType::Error,
                range: (22, 25),
            }],
        }],
    };

    println!("{}", DisplayList::from(snippet));
}
