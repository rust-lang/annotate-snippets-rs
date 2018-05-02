extern crate annotate_snippets;

use annotate_snippets::display_list::DisplayList;
use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet, TitleAnnotation};

fn main() {
    let snippet = Snippet {
        title: Some(TitleAnnotation {
            label: Some("mismatched types".to_string()),
            id: None,
            annotation_type: AnnotationType::Error,
        }),
        slices: vec![
            Slice {
                source: "Foo".to_string(),
                line_start: 51,
                origin: Some("src/format.rs".to_string()),
                fold: false,
                annotations: vec![],
            },
            Slice {
                source: "Faa".to_string(),
                line_start: 129,
                origin: Some("src/display.rs".to_string()),
                fold: false,
                annotations: vec![],
            },
        ],
    };

    println!("{}", DisplayList::from(snippet));
}
