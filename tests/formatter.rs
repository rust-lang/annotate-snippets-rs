use annotate_snippets::{Annotation, AnnotationType, Renderer, Slice, Snippet, SourceAnnotation};

#[test]
fn test_i_29() {
    let snippets = Snippet {
        title: Some(Annotation {
            id: None,
            label: Some("oops"),
            annotation_type: AnnotationType::Error,
        }),
        footer: vec![],
        slices: vec![Slice {
            source: "First line\r\nSecond oops line",
            line_start: 1,
            origin: Some("<current file>"),
            annotations: vec![SourceAnnotation {
                range: (19, 23),
                label: "oops",
                annotation_type: AnnotationType::Error,
            }],
            fold: true,
        }],
    };
    let expected = r#"error: oops
 --> <current file>:2:8
  |
1 | First line
2 | Second oops line
  |        ^^^^ oops
  |"#;

    let renderer = Renderer::plain();
    assert_eq!(renderer.render(snippets).to_string(), expected);
}

#[test]
fn test_point_to_double_width_characters() {
    let snippets = Snippet {
        slices: vec![Slice {
            source: "こんにちは、世界",
            line_start: 1,
            origin: Some("<current file>"),
            annotations: vec![SourceAnnotation {
                range: (6, 8),
                label: "world",
                annotation_type: AnnotationType::Error,
            }],
            fold: false,
        }],
        title: None,
        footer: vec![],
    };

    let expected = r#" --> <current file>:1:7
  |
1 | こんにちは、世界
  |             ^^^^ world
  |"#;

    let renderer = Renderer::plain();
    assert_eq!(renderer.render(snippets).to_string(), expected);
}

#[test]
fn test_point_to_double_width_characters_across_lines() {
    let snippets = Snippet {
        slices: vec![Slice {
            source: "おはよう\nございます",
            line_start: 1,
            origin: Some("<current file>"),
            annotations: vec![SourceAnnotation {
                range: (2, 8),
                label: "Good morning",
                annotation_type: AnnotationType::Error,
            }],
            fold: false,
        }],
        title: None,
        footer: vec![],
    };

    let expected = r#" --> <current file>:1:3
  |
1 |   おはよう
  |  _____^
2 | | ございます
  | |______^ Good morning
  |"#;

    let renderer = Renderer::plain();
    assert_eq!(renderer.render(snippets).to_string(), expected);
}

#[test]
fn test_point_to_double_width_characters_multiple() {
    let snippets = Snippet {
        slices: vec![Slice {
            source: "お寿司\n食べたい🍣",
            line_start: 1,
            origin: Some("<current file>"),
            annotations: vec![
                SourceAnnotation {
                    range: (0, 3),
                    label: "Sushi1",
                    annotation_type: AnnotationType::Error,
                },
                SourceAnnotation {
                    range: (6, 8),
                    label: "Sushi2",
                    annotation_type: AnnotationType::Note,
                },
            ],
            fold: false,
        }],
        title: None,
        footer: vec![],
    };

    let expected = r#" --> <current file>:1:1
  |
1 | お寿司
  | ^^^^^^ Sushi1
2 | 食べたい🍣
  |     ---- note: Sushi2
  |"#;

    let renderer = Renderer::plain();
    assert_eq!(renderer.render(snippets).to_string(), expected);
}

#[test]
fn test_point_to_double_width_characters_mixed() {
    let snippets = Snippet {
        slices: vec![Slice {
            source: "こんにちは、新しいWorld！",
            line_start: 1,
            origin: Some("<current file>"),
            annotations: vec![SourceAnnotation {
                range: (6, 14),
                label: "New world",
                annotation_type: AnnotationType::Error,
            }],
            fold: false,
        }],
        title: None,
        footer: vec![],
    };

    let expected = r#" --> <current file>:1:7
  |
1 | こんにちは、新しいWorld！
  |             ^^^^^^^^^^^ New world
  |"#;

    let renderer = Renderer::plain();
    assert_eq!(renderer.render(snippets).to_string(), expected);
}
