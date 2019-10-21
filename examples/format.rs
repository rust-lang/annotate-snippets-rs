use annotate_snippets::*;
use std::io;

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
                data: source,
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
    };

    let formatted = format(&snippet, &());
    renderer::Ascii::ansi()
        .render(&formatted, &(), &mut io::stdout().lock())
        .unwrap();
}
