use annotate_snippets::display_list::DisplayList;
use annotate_snippets::formatter::DisplayListFormatter;
use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation};

fn main() {
    let snippet = Snippet {
        title: Some(Annotation {
            label: Some("expected type, found `22`".to_string()),
            id: None,
            annotation_type: AnnotationType::Error,
        }),
        footer: vec![],
        slices: vec![Slice {
            source: r#"                annotations: vec![SourceAnnotation {
                label: "expected struct `annotate_snippets::snippet::Slice`, found reference"
                    .to_string(),
                range: <22, 25>,"#
                .to_string(),
            line_start: 26,
            origin: Some("examples/footer.rs".to_string()),
            fold: true,
            annotations: vec![
                SourceAnnotation {
                    label: "".to_string(),
                    annotation_type: AnnotationType::Error,
                    range: (208, 210),
                },
                SourceAnnotation {
                    label: "while parsing this struct".to_string(),
                    annotation_type: AnnotationType::Info,
                    range: (34, 50),
                },
            ],
        }],
    };

    let dl = DisplayList::from(snippet);
    let dlf = DisplayListFormatter::new(true, false);
    println!("{}", dlf.format(&dl));
}
