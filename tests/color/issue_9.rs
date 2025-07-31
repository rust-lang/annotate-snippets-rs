use annotate_snippets::{AnnotationKind, Group, Level, Renderer, Snippet};

use snapbox::{assert_data_eq, file};

#[test]
fn case() {
    let source = r#"let x = vec![1];


let y = x;

x;
"#;

    let input = &[Group::with_title(Level::ERROR.title("expected one of `.`, `;`, `?`, or an operator, found `for`"))
        .element(
            Snippet::source(source)
                .path("/code/rust/src/test/ui/annotate-snippet/suggestion.rs")
                .line_start(4)
                .annotation(AnnotationKind::Context.span(4..5).label("move occurs because `x` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait"))
                .annotation(AnnotationKind::Context.span(27..28).label("value moved here"))
                .annotation(AnnotationKind::Primary.span(31..32).label("value used here after move"))
        )
    ];
    let expected = file!["issue_9.term.svg"];
    let renderer = Renderer::styled();
    assert_data_eq!(renderer.render(input), expected);
}
