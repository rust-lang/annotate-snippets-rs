use annotate_snippets::{AnnotationKind, Group, Level, Renderer, Snippet};

use snapbox::{assert_data_eq, file};

#[test]
fn case() {
    let source = r#"	let _: () = (); let _: () = (); let _: () = (); let _: () = (); let _: () = (); let _: () = (); let _: () = (); let _: () = (); let _: () = (); let _: () = (); let _: () = (); let _: () = (); let _: () = (); let _: () = (); let _: () = 42; let _: () = (); let _: () = (); let _: () = (); let _: () = (); let _: () = (); let _: () = (); let _: () = (); let _: () = ();
"#;

    let input = &[Group::new()
        .element(Level::ERROR.title("mismatched types").id("E0308"))
        .element(
            Snippet::source(source)
                .path("$DIR/non-whitespace-trimming.rs")
                .line_start(4)
                .annotation(
                    AnnotationKind::Primary
                        .span(237..239)
                        .label("expected `()`, found integer"),
                )
                .annotation(
                    AnnotationKind::Primary
                        .span(232..234)
                        .label("expected due to this"),
                ),
        )];
    let expected = file!["strip_line_non_ws.term.svg"];
    let renderer = Renderer::styled().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input), expected);
}
