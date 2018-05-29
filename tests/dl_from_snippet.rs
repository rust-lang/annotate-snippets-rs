extern crate annotate_snippets;

use annotate_snippets::display_list as dl;
use annotate_snippets::snippet;

#[test]
fn test_format_title() {
    let input = snippet::Snippet {
        title: Some(snippet::Annotation {
            id: Some("E0001".to_string()),
            label: Some("This is a title".to_string()),
            annotation_type: snippet::AnnotationType::Error,
        }),
        footer: vec![],
        slices: vec![],
    };
    let output = dl::DisplayList {
        body: vec![dl::DisplayLine::Raw(dl::DisplayRawLine::Annotation {
            annotation: dl::Annotation {
                annotation_type: dl::DisplayAnnotationType::Error,
                id: Some("E0001".to_string()),
                label: vec![dl::DisplayTextFragment {
                    content: "This is a title".to_string(),
                    style: dl::DisplayTextStyle::Emphasis,
                }],
            },
            source_aligned: false,
            continuation: false,
        })],
    };
    assert_eq!(dl::DisplayList::from(input), output);
}

#[test]
fn test_format_slice() {
    let input = snippet::Snippet {
        title: None,
        footer: vec![],
        slices: vec![snippet::Slice {
            source: "This is line 1\nThis is line 2".to_string(),
            line_start: 5402,
            origin: None,
            annotations: vec![],
            fold: false,
        }],
    };
    let output = dl::DisplayList {
        body: vec![
            dl::DisplayLine::Source {
                lineno: None,
                inline_marks: vec![],
                line: dl::DisplaySourceLine::Empty,
            },
            dl::DisplayLine::Source {
                lineno: Some(5402),
                inline_marks: vec![],
                line: dl::DisplaySourceLine::Content {
                    text: "This is line 1".to_string(),
                    range: (0, 15),
                },
            },
            dl::DisplayLine::Source {
                lineno: Some(5403),
                inline_marks: vec![],
                line: dl::DisplaySourceLine::Content {
                    text: "This is line 2".to_string(),
                    range: (16, 31),
                },
            },
            dl::DisplayLine::Source {
                lineno: None,
                inline_marks: vec![],
                line: dl::DisplaySourceLine::Empty,
            },
        ],
    };
    assert_eq!(dl::DisplayList::from(input), output);
}

#[test]
fn test_format_slices_continuation() {
    let input = snippet::Snippet {
        title: None,
        footer: vec![],
        slices: vec![
            snippet::Slice {
                source: "This is slice 1".to_string(),
                line_start: 5402,
                origin: Some("file1.rs".to_string()),
                annotations: vec![],
                fold: false,
            },
            snippet::Slice {
                source: "This is slice 2".to_string(),
                line_start: 2,
                origin: Some("file2.rs".to_string()),
                annotations: vec![],
                fold: false,
            },
        ],
    };
    let output = dl::DisplayList {
        body: vec![
            dl::DisplayLine::Raw(dl::DisplayRawLine::Origin {
                path: "file1.rs".to_string(),
                pos: None,
                header_type: dl::DisplayHeaderType::Initial,
            }),
            dl::DisplayLine::Source {
                lineno: None,
                inline_marks: vec![],
                line: dl::DisplaySourceLine::Empty,
            },
            dl::DisplayLine::Source {
                lineno: Some(5402),
                inline_marks: vec![],
                line: dl::DisplaySourceLine::Content {
                    text: "This is slice 1".to_string(),
                    range: (0, 16),
                },
            },
            dl::DisplayLine::Source {
                lineno: None,
                inline_marks: vec![],
                line: dl::DisplaySourceLine::Empty,
            },
            dl::DisplayLine::Raw(dl::DisplayRawLine::Origin {
                path: "file2.rs".to_string(),
                pos: None,
                header_type: dl::DisplayHeaderType::Continuation,
            }),
            dl::DisplayLine::Source {
                lineno: None,
                inline_marks: vec![],
                line: dl::DisplaySourceLine::Empty,
            },
            dl::DisplayLine::Source {
                lineno: Some(2),
                inline_marks: vec![],
                line: dl::DisplaySourceLine::Content {
                    text: "This is slice 2".to_string(),
                    range: (0, 16),
                },
            },
            dl::DisplayLine::Source {
                lineno: None,
                inline_marks: vec![],
                line: dl::DisplaySourceLine::Empty,
            },
        ],
    };
    assert_eq!(dl::DisplayList::from(input), output);
}

#[test]
fn test_format_slice_annotation_standalone() {
    let input = snippet::Snippet {
        title: None,
        footer: vec![],
        slices: vec![snippet::Slice {
            source: "This is line 1\nThis is line 2".to_string(),
            line_start: 5402,
            origin: None,
            annotations: vec![snippet::SourceAnnotation {
                range: (22, 24),
                label: "Test annotation".to_string(),
                annotation_type: snippet::AnnotationType::Info,
            }],
            fold: false,
        }],
    };
    let output = dl::DisplayList {
        body: vec![
            dl::DisplayLine::Source {
                lineno: None,
                inline_marks: vec![],
                line: dl::DisplaySourceLine::Empty,
            },
            dl::DisplayLine::Source {
                lineno: Some(5402),
                inline_marks: vec![],
                line: dl::DisplaySourceLine::Content {
                    text: "This is line 1".to_string(),
                    range: (0, 15),
                },
            },
            dl::DisplayLine::Source {
                lineno: Some(5403),
                inline_marks: vec![],
                line: dl::DisplaySourceLine::Content {
                    text: "This is line 2".to_string(),
                    range: (16, 31),
                },
            },
            dl::DisplayLine::Source {
                lineno: None,
                inline_marks: vec![],
                line: dl::DisplaySourceLine::Annotation {
                    annotation: dl::Annotation {
                        annotation_type: dl::DisplayAnnotationType::Info,
                        id: None,
                        label: vec![dl::DisplayTextFragment {
                            content: "Test annotation".to_string(),
                            style: dl::DisplayTextStyle::Regular,
                        }],
                    },
                    range: (6, 8),
                    annotation_type: dl::DisplayAnnotationType::Info,
                    annotation_part: dl::DisplayAnnotationPart::Standalone,
                },
            },
            dl::DisplayLine::Source {
                lineno: None,
                inline_marks: vec![],
                line: dl::DisplaySourceLine::Empty,
            },
        ],
    };
    assert_eq!(dl::DisplayList::from(input), output);
}

#[test]
fn test_format_label() {
    let input = snippet::Snippet {
        title: None,
        footer: vec![snippet::Annotation {
            id: None,
            label: Some("This __is__ a title".to_string()),
            annotation_type: snippet::AnnotationType::Error,
        }],
        slices: vec![],
    };
    let output = dl::DisplayList {
        body: vec![dl::DisplayLine::Raw(dl::DisplayRawLine::Annotation {
            annotation: dl::Annotation {
                annotation_type: dl::DisplayAnnotationType::Error,
                id: None,
                label: vec![
                    dl::DisplayTextFragment {
                        content: "This ".to_string(),
                        style: dl::DisplayTextStyle::Regular,
                    },
                    dl::DisplayTextFragment {
                        content: "is".to_string(),
                        style: dl::DisplayTextStyle::Emphasis,
                    },
                    dl::DisplayTextFragment {
                        content: " a title".to_string(),
                        style: dl::DisplayTextStyle::Regular,
                    },
                ],
            },
            source_aligned: true,
            continuation: false,
        })],
    };
    assert_eq!(dl::DisplayList::from(input), output);
}
