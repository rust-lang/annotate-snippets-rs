#[cfg(feature = "ansi_term")]
use annotate_snippets::renderers::ascii_default::styles::color::Style as ColorStyle;
#[cfg(feature = "termcolor")]
use annotate_snippets::renderers::ascii_default::styles::color2::Style as ColorStyle;
#[cfg(all(not(feature = "ansi_term"), not(feature = "termcolor")))]
use annotate_snippets::renderers::ascii_default::styles::plain::Style as PlainStyle;
use annotate_snippets::renderers::ascii_default::Renderer as AsciiRenderer;
use annotate_snippets::DisplayList;
use annotate_snippets::{Annotation, AnnotationType, SourceAnnotation};
use annotate_snippets::{Slice, Snippet};

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
    let dl = DisplayList::from(&snippet);

    #[cfg(all(not(feature = "ansi_term"), not(feature = "termcolor")))]
    let r = AsciiRenderer::<PlainStyle>::new();
    #[cfg(feature = "ansi_term")]
    let r = AsciiRenderer::<ColorStyle>::new();
    #[cfg(feature = "termcolor")]
    let r = AsciiRenderer::<ColorStyle>::new();
    let mut s: Vec<u8> = Vec::new();
    r.fmt(&mut s, &dl).unwrap();
    println!("{}", std::str::from_utf8(&s).unwrap());
}
