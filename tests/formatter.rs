use annotate_snippets::display_list::*;
use annotate_snippets::formatter::DisplayListFormatter;

#[test]
fn test_source_empty() {
    let dl = DisplayList::from(vec![DisplayLine::Source {
        lineno: None,
        inline_marks: vec![],
        line: DisplaySourceLine::Empty,
    }]);

    let dlf = DisplayListFormatter::new(false, false);

    assert_eq!(dlf.format(&dl), " |");
}

#[test]
fn test_source_content() {
    let dl = DisplayList::from(vec![
        DisplayLine::Source {
            lineno: Some(56),
            inline_marks: vec![],
            line: DisplaySourceLine::Content {
                text: "This is an example".to_string(),
                range: (0, 19),
            },
        },
        DisplayLine::Source {
            lineno: Some(57),
            inline_marks: vec![],
            line: DisplaySourceLine::Content {
                text: "of content lines".to_string(),
                range: (0, 19),
            },
        },
    ]);

    let dlf = DisplayListFormatter::new(false, false);

    assert_eq!(
        dlf.format(&dl),
        "56 | This is an example\n57 | of content lines"
    );
}

#[test]
fn test_source_annotation_standalone_singleline() {
    let dl = DisplayList::from(vec![DisplayLine::Source {
        lineno: None,
        inline_marks: vec![],
        line: DisplaySourceLine::Annotation {
            range: (0, 5),
            annotation: Annotation {
                annotation_type: DisplayAnnotationType::None,
                id: None,
                label: vec![DisplayTextFragment {
                    content: String::from("Example string"),
                    style: DisplayTextStyle::Regular,
                }],
            },
            annotation_type: DisplayAnnotationType::Error,
            annotation_part: DisplayAnnotationPart::Standalone,
        },
    }]);

    let dlf = DisplayListFormatter::new(false, false);

    assert_eq!(dlf.format(&dl), " | ^^^^^ Example string");
}

#[test]
fn test_source_annotation_standalone_multiline() {
    let dl = DisplayList::from(vec![
        DisplayLine::Source {
            lineno: None,
            inline_marks: vec![],
            line: DisplaySourceLine::Annotation {
                range: (0, 5),
                annotation: Annotation {
                    annotation_type: DisplayAnnotationType::Help,
                    id: None,
                    label: vec![DisplayTextFragment {
                        content: String::from("Example string"),
                        style: DisplayTextStyle::Regular,
                    }],
                },
                annotation_type: DisplayAnnotationType::Warning,
                annotation_part: DisplayAnnotationPart::Standalone,
            },
        },
        DisplayLine::Source {
            lineno: None,
            inline_marks: vec![],
            line: DisplaySourceLine::Annotation {
                range: (0, 5),
                annotation: Annotation {
                    annotation_type: DisplayAnnotationType::Help,
                    id: None,
                    label: vec![DisplayTextFragment {
                        content: String::from("Second line"),
                        style: DisplayTextStyle::Regular,
                    }],
                },
                annotation_type: DisplayAnnotationType::Warning,
                annotation_part: DisplayAnnotationPart::LabelContinuation,
            },
        },
    ]);

    let dlf = DisplayListFormatter::new(false, false);

    assert_eq!(
        dlf.format(&dl),
        " | ----- help: Example string\n |             Second line"
    );
}

#[test]
fn test_source_annotation_standalone_multi_annotation() {
    let dl = DisplayList::from(vec![
        DisplayLine::Source {
            lineno: None,
            inline_marks: vec![],
            line: DisplaySourceLine::Annotation {
                range: (0, 5),
                annotation: Annotation {
                    annotation_type: DisplayAnnotationType::Info,
                    id: None,
                    label: vec![DisplayTextFragment {
                        content: String::from("Example string"),
                        style: DisplayTextStyle::Regular,
                    }],
                },
                annotation_type: DisplayAnnotationType::Note,
                annotation_part: DisplayAnnotationPart::Standalone,
            },
        },
        DisplayLine::Source {
            lineno: None,
            inline_marks: vec![],
            line: DisplaySourceLine::Annotation {
                range: (0, 5),
                annotation: Annotation {
                    annotation_type: DisplayAnnotationType::Info,
                    id: None,
                    label: vec![DisplayTextFragment {
                        content: String::from("Second line"),
                        style: DisplayTextStyle::Regular,
                    }],
                },
                annotation_type: DisplayAnnotationType::Note,
                annotation_part: DisplayAnnotationPart::LabelContinuation,
            },
        },
        DisplayLine::Source {
            lineno: None,
            inline_marks: vec![],
            line: DisplaySourceLine::Annotation {
                range: (0, 5),
                annotation: Annotation {
                    annotation_type: DisplayAnnotationType::Warning,
                    id: None,
                    label: vec![DisplayTextFragment {
                        content: String::from("This is a note"),
                        style: DisplayTextStyle::Regular,
                    }],
                },
                annotation_type: DisplayAnnotationType::Note,
                annotation_part: DisplayAnnotationPart::Consequitive,
            },
        },
        DisplayLine::Source {
            lineno: None,
            inline_marks: vec![],
            line: DisplaySourceLine::Annotation {
                range: (0, 5),
                annotation: Annotation {
                    annotation_type: DisplayAnnotationType::Warning,
                    id: None,
                    label: vec![DisplayTextFragment {
                        content: String::from("Second line of the warning"),
                        style: DisplayTextStyle::Regular,
                    }],
                },
                annotation_type: DisplayAnnotationType::Note,
                annotation_part: DisplayAnnotationPart::LabelContinuation,
            },
        },
        DisplayLine::Source {
            lineno: None,
            inline_marks: vec![],
            line: DisplaySourceLine::Annotation {
                range: (0, 5),
                annotation: Annotation {
                    annotation_type: DisplayAnnotationType::Info,
                    id: None,
                    label: vec![DisplayTextFragment {
                        content: String::from("This is an info"),
                        style: DisplayTextStyle::Regular,
                    }],
                },
                annotation_type: DisplayAnnotationType::Info,
                annotation_part: DisplayAnnotationPart::Standalone,
            },
        },
        DisplayLine::Source {
            lineno: None,
            inline_marks: vec![],
            line: DisplaySourceLine::Annotation {
                range: (0, 5),
                annotation: Annotation {
                    annotation_type: DisplayAnnotationType::Help,
                    id: None,
                    label: vec![DisplayTextFragment {
                        content: String::from("This is help"),
                        style: DisplayTextStyle::Regular,
                    }],
                },
                annotation_type: DisplayAnnotationType::Help,
                annotation_part: DisplayAnnotationPart::Standalone,
            },
        },
        DisplayLine::Source {
            lineno: None,
            inline_marks: vec![],
            line: DisplaySourceLine::Annotation {
                range: (0, 0),
                annotation: Annotation {
                    annotation_type: DisplayAnnotationType::None,
                    id: None,
                    label: vec![DisplayTextFragment {
                        content: String::from("This is an annotation of type none"),
                        style: DisplayTextStyle::Regular,
                    }],
                },
                annotation_type: DisplayAnnotationType::None,
                annotation_part: DisplayAnnotationPart::Standalone,
            },
        },
    ]);

    let dlf = DisplayListFormatter::new(false, false);

    assert_eq!(dlf.format(&dl), " | ----- info: Example string\n |             Second line\n |       warning: This is a note\n |                Second line of the warning\n | ----- info: This is an info\n | ----- help: This is help\n |  This is an annotation of type none");
}

#[test]
fn test_fold_line() {
    let dl = DisplayList::from(vec![
        DisplayLine::Source {
            lineno: Some(5),
            inline_marks: vec![],
            line: DisplaySourceLine::Content {
                text: "This is line 5".to_string(),
                range: (0, 19),
            },
        },
        DisplayLine::Fold {
            inline_marks: vec![],
        },
        DisplayLine::Source {
            lineno: Some(10021),
            inline_marks: vec![],
            line: DisplaySourceLine::Content {
                text: "... and now we're at line 10021".to_string(),
                range: (0, 19),
            },
        },
    ]);

    let dlf = DisplayListFormatter::new(false, false);

    assert_eq!(
        dlf.format(&dl),
        "    5 | This is line 5\n...\n10021 | ... and now we're at line 10021"
    );
}

#[test]
fn test_raw_origin_initial_nopos() {
    let dl = DisplayList::from(vec![DisplayLine::Raw(DisplayRawLine::Origin {
        path: "src/test.rs".to_string(),
        pos: None,
        header_type: DisplayHeaderType::Initial,
    })]);

    let dlf = DisplayListFormatter::new(false, false);

    assert_eq!(dlf.format(&dl), "--> src/test.rs");
}

#[test]
fn test_raw_origin_initial_pos() {
    let dl = DisplayList::from(vec![DisplayLine::Raw(DisplayRawLine::Origin {
        path: "src/test.rs".to_string(),
        pos: Some((23, 15)),
        header_type: DisplayHeaderType::Initial,
    })]);

    let dlf = DisplayListFormatter::new(false, false);

    assert_eq!(dlf.format(&dl), "--> src/test.rs:23:15");
}

#[test]
fn test_raw_origin_continuation() {
    let dl = DisplayList::from(vec![DisplayLine::Raw(DisplayRawLine::Origin {
        path: "src/test.rs".to_string(),
        pos: Some((23, 15)),
        header_type: DisplayHeaderType::Continuation,
    })]);

    let dlf = DisplayListFormatter::new(false, false);

    assert_eq!(dlf.format(&dl), "::: src/test.rs:23:15");
}

#[test]
fn test_raw_annotation_unaligned() {
    let dl = DisplayList::from(vec![DisplayLine::Raw(DisplayRawLine::Annotation {
        annotation: Annotation {
            annotation_type: DisplayAnnotationType::Error,
            id: Some("E0001".to_string()),
            label: vec![DisplayTextFragment {
                content: String::from("This is an error"),
                style: DisplayTextStyle::Regular,
            }],
        },
        source_aligned: false,
        continuation: false,
    })]);

    let dlf = DisplayListFormatter::new(false, false);

    assert_eq!(dlf.format(&dl), "error[E0001]: This is an error");
}

#[test]
fn test_raw_annotation_unaligned_multiline() {
    let dl = DisplayList::from(vec![
        DisplayLine::Raw(DisplayRawLine::Annotation {
            annotation: Annotation {
                annotation_type: DisplayAnnotationType::Warning,
                id: Some("E0001".to_string()),
                label: vec![DisplayTextFragment {
                    content: String::from("This is an error"),
                    style: DisplayTextStyle::Regular,
                }],
            },
            source_aligned: false,
            continuation: false,
        }),
        DisplayLine::Raw(DisplayRawLine::Annotation {
            annotation: Annotation {
                annotation_type: DisplayAnnotationType::Warning,
                id: Some("E0001".to_string()),
                label: vec![DisplayTextFragment {
                    content: String::from("Second line of the error"),
                    style: DisplayTextStyle::Regular,
                }],
            },
            source_aligned: false,
            continuation: true,
        }),
    ]);

    let dlf = DisplayListFormatter::new(false, false);

    assert_eq!(
        dlf.format(&dl),
        "warning[E0001]: This is an error\n                Second line of the error"
    );
}

#[test]
fn test_raw_annotation_aligned() {
    let dl = DisplayList::from(vec![DisplayLine::Raw(DisplayRawLine::Annotation {
        annotation: Annotation {
            annotation_type: DisplayAnnotationType::Error,
            id: Some("E0001".to_string()),
            label: vec![DisplayTextFragment {
                content: String::from("This is an error"),
                style: DisplayTextStyle::Regular,
            }],
        },
        source_aligned: true,
        continuation: false,
    })]);

    let dlf = DisplayListFormatter::new(false, false);

    assert_eq!(dlf.format(&dl), " = error[E0001]: This is an error");
}

#[test]
fn test_raw_annotation_aligned_multiline() {
    let dl = DisplayList::from(vec![
        DisplayLine::Raw(DisplayRawLine::Annotation {
            annotation: Annotation {
                annotation_type: DisplayAnnotationType::Warning,
                id: Some("E0001".to_string()),
                label: vec![DisplayTextFragment {
                    content: String::from("This is an error"),
                    style: DisplayTextStyle::Regular,
                }],
            },
            source_aligned: true,
            continuation: false,
        }),
        DisplayLine::Raw(DisplayRawLine::Annotation {
            annotation: Annotation {
                annotation_type: DisplayAnnotationType::Warning,
                id: Some("E0001".to_string()),
                label: vec![DisplayTextFragment {
                    content: String::from("Second line of the error"),
                    style: DisplayTextStyle::Regular,
                }],
            },
            source_aligned: true,
            continuation: true,
        }),
    ]);

    let dlf = DisplayListFormatter::new(false, false);

    assert_eq!(
        dlf.format(&dl),
        " = warning[E0001]: This is an error\n                   Second line of the error"
    );
}

#[test]
fn test_different_annotation_types() {
    let dl = DisplayList::from(vec![
        DisplayLine::Raw(DisplayRawLine::Annotation {
            annotation: Annotation {
                annotation_type: DisplayAnnotationType::Note,
                id: None,
                label: vec![DisplayTextFragment {
                    content: String::from("This is a note"),
                    style: DisplayTextStyle::Regular,
                }],
            },
            source_aligned: false,
            continuation: false,
        }),
        DisplayLine::Raw(DisplayRawLine::Annotation {
            annotation: Annotation {
                annotation_type: DisplayAnnotationType::None,
                id: None,
                label: vec![DisplayTextFragment {
                    content: String::from("This is just a string"),
                    style: DisplayTextStyle::Regular,
                }],
            },
            source_aligned: false,
            continuation: false,
        }),
        DisplayLine::Raw(DisplayRawLine::Annotation {
            annotation: Annotation {
                annotation_type: DisplayAnnotationType::None,
                id: None,
                label: vec![DisplayTextFragment {
                    content: String::from("Second line of none type annotation"),
                    style: DisplayTextStyle::Regular,
                }],
            },
            source_aligned: false,
            continuation: true,
        }),
    ]);

    let dlf = DisplayListFormatter::new(false, false);

    assert_eq!(
        dlf.format(&dl),
        "note: This is a note\nThis is just a string\n  Second line of none type annotation",
    );
}

#[test]
fn test_inline_marks_empty_line() {
    let dl = DisplayList::from(vec![DisplayLine::Source {
        lineno: None,
        inline_marks: vec![DisplayMark {
            mark_type: DisplayMarkType::AnnotationThrough,
            annotation_type: DisplayAnnotationType::Error,
        }],
        line: DisplaySourceLine::Empty,
    }]);

    let dlf = DisplayListFormatter::new(false, false);

    assert_eq!(dlf.format(&dl), " | |",);
}

#[test]
fn test_anon_lines() {
    let dl = DisplayList::from(vec![
        DisplayLine::Source {
            lineno: Some(56),
            inline_marks: vec![],
            line: DisplaySourceLine::Content {
                text: "This is an example".to_string(),
                range: (0, 19),
            },
        },
        DisplayLine::Source {
            lineno: Some(57),
            inline_marks: vec![],
            line: DisplaySourceLine::Content {
                text: "of content lines".to_string(),
                range: (0, 19),
            },
        },
        DisplayLine::Source {
            lineno: None,
            inline_marks: vec![],
            line: DisplaySourceLine::Empty,
        },
        DisplayLine::Source {
            lineno: None,
            inline_marks: vec![],
            line: DisplaySourceLine::Content {
                text: "abc".to_string(),
                range: (0, 19),
            },
        },
    ]);

    let dlf = DisplayListFormatter::new(false, true);

    assert_eq!(
        dlf.format(&dl),
        "LL | This is an example\nLL | of content lines\n   |\n   | abc"
    );
}

#[test]
fn test_raw_origin_initial_pos_anon_lines() {
    let dl = DisplayList::from(vec![DisplayLine::Raw(DisplayRawLine::Origin {
        path: "src/test.rs".to_string(),
        pos: Some((23, 15)),
        header_type: DisplayHeaderType::Initial,
    })]);

    let dlf = DisplayListFormatter::new(false, true);

    // Using anonymized_line_numbers should not affect the inital position
    assert_eq!(dlf.format(&dl), "--> src/test.rs:23:15");
}
