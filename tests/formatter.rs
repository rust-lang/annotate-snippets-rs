use annotate_snippets::{Level, Renderer, Snippet};

use snapbox::{assert_eq, str};

#[test]
fn test_i_29() {
    let snippets = Level::Error.title("oops").snippet(
        Snippet::source("First line\r\nSecond oops line")
            .origin("<current file>")
            .annotation(Level::Error.span(19..23).label("oops"))
            .fold(true),
    );
    let expected = str![[r#"
error: oops
 --> <current file>:2:8
  |
2 | Second oops line
  |        ^^^^ oops
  |"#]]
    .indent(false);

    let renderer = Renderer::plain();
    assert_eq(expected, renderer.render(snippets).to_string());
}

#[test]
fn test_point_to_double_width_characters() {
    let snippets = Level::Error.title("").snippet(
        Snippet::source("こんにちは、世界")
            .origin("<current file>")
            .annotation(Level::Error.span(18..24).label("world")),
    );

    let expected = str![[r#"
error
 --> <current file>:1:7
  |
1 | こんにちは、世界
  |             ^^^^ world
  |"#]]
    .indent(false);

    let renderer = Renderer::plain();
    assert_eq(expected, renderer.render(snippets).to_string());
}

#[test]
fn test_point_to_double_width_characters_across_lines() {
    let snippets = Level::Error.title("").snippet(
        Snippet::source("おはよう\nございます")
            .origin("<current file>")
            .annotation(Level::Error.span(6..22).label("Good morning")),
    );

    let expected = str![[r#"
error
 --> <current file>:1:3
  |
1 |   おはよう
  |  _____^
2 | | ございます
  | |______^ Good morning
  |"#]]
    .indent(false);

    let renderer = Renderer::plain();
    assert_eq(expected, renderer.render(snippets).to_string());
}

#[test]
fn test_point_to_double_width_characters_multiple() {
    let snippets = Level::Error.title("").snippet(
        Snippet::source("お寿司\n食べたい🍣")
            .origin("<current file>")
            .annotation(Level::Error.span(0..9).label("Sushi1"))
            .annotation(Level::Note.span(16..22).label("Sushi2")),
    );

    let expected = str![[r#"
error
 --> <current file>:1:1
  |
1 | お寿司
  | ^^^^^^ Sushi1
2 | 食べたい🍣
  |     ---- note: Sushi2
  |"#]]
    .indent(false);

    let renderer = Renderer::plain();
    assert_eq(expected, renderer.render(snippets).to_string());
}

#[test]
fn test_point_to_double_width_characters_mixed() {
    let snippets = Level::Error.title("").snippet(
        Snippet::source("こんにちは、新しいWorld！")
            .origin("<current file>")
            .annotation(Level::Error.span(18..32).label("New world")),
    );

    let expected = str![[r#"
error
 --> <current file>:1:7
  |
1 | こんにちは、新しいWorld！
  |             ^^^^^^^^^^^ New world
  |"#]]
    .indent(false);

    let renderer = Renderer::plain();
    assert_eq(expected, renderer.render(snippets).to_string());
}

#[test]
fn test_format_title() {
    let input = Level::Error.title("This is a title").id("E0001");

    let expected = str![r#"error[E0001]: This is a title"#];
    let renderer = Renderer::plain();
    assert_eq(expected, renderer.render(input).to_string());
}

#[test]
fn test_format_snippet_only() {
    let source = "This is line 1\nThis is line 2";
    let input = Level::Error
        .title("")
        .snippet(Snippet::source(source).line_start(5402));

    let expected = str![[r#"
error
     |
5402 | This is line 1
5403 | This is line 2
     |"#]]
    .indent(false);
    let renderer = Renderer::plain();
    assert_eq(expected, renderer.render(input).to_string());
}

#[test]
fn test_format_snippets_continuation() {
    let src_0 = "This is slice 1";
    let src_1 = "This is slice 2";
    let input = Level::Error
        .title("")
        .snippet(Snippet::source(src_0).line_start(5402).origin("file1.rs"))
        .snippet(Snippet::source(src_1).line_start(2).origin("file2.rs"));
    let expected = str![[r#"
error
    --> file1.rs
     |
5402 | This is slice 1
     |
    ::: file2.rs
     |
   2 | This is slice 2
     |"#]]
    .indent(false);
    let renderer = Renderer::plain();
    assert_eq(expected, renderer.render(input).to_string());
}

#[test]
fn test_format_snippet_annotation_standalone() {
    let line_1 = "This is line 1";
    let line_2 = "This is line 2";
    let source = [line_1, line_2].join("\n");
    // In line 2
    let range = 22..24;
    let input = Level::Error.title("").snippet(
        Snippet::source(&source)
            .line_start(5402)
            .annotation(Level::Info.span(range.clone()).label("Test annotation")),
    );
    let expected = str![[r#"
error
     |
5402 | This is line 1
5403 | This is line 2
     |        -- info: Test annotation
     |"#]]
    .indent(false);
    let renderer = Renderer::plain();
    assert_eq(expected, renderer.render(input).to_string());
}

#[test]
fn test_format_footer_title() {
    let input = Level::Error
        .title("")
        .footer(Level::Error.title("This __is__ a title"));
    let expected = str![[r#"
error
 = error: This __is__ a title"#]]
    .indent(false);
    let renderer = Renderer::plain();
    assert_eq(expected, renderer.render(input).to_string());
}

#[test]
#[should_panic]
fn test_i26() {
    let source = "short";
    let label = "label";
    let input = Level::Error.title("").snippet(
        Snippet::source(source)
            .line_start(0)
            .annotation(Level::Error.span(0..source.len() + 2).label(label)),
    );
    let renderer = Renderer::plain();
    let _ = renderer.render(input).to_string();
}

#[test]
fn test_source_content() {
    let source = "This is an example\nof content lines";
    let input = Level::Error
        .title("")
        .snippet(Snippet::source(source).line_start(56));
    let expected = str![[r#"
error
   |
56 | This is an example
57 | of content lines
   |"#]]
    .indent(false);
    let renderer = Renderer::plain();
    assert_eq(expected, renderer.render(input).to_string());
}

#[test]
fn test_source_annotation_standalone_singleline() {
    let source = "tests";
    let input = Level::Error.title("").snippet(
        Snippet::source(source)
            .line_start(1)
            .annotation(Level::Help.span(0..5).label("Example string")),
    );
    let expected = str![[r#"
error
  |
1 | tests
  | ----- help: Example string
  |"#]]
    .indent(false);
    let renderer = Renderer::plain();
    assert_eq(expected, renderer.render(input).to_string());
}

#[test]
fn test_source_annotation_standalone_multiline() {
    let source = "tests";
    let input = Level::Error.title("").snippet(
        Snippet::source(source)
            .line_start(1)
            .annotation(Level::Help.span(0..5).label("Example string"))
            .annotation(Level::Help.span(0..5).label("Second line")),
    );
    let expected = str![[r#"
error
  |
1 | tests
  | ----- help: Example string
  | ----- help: Second line
  |"#]]
    .indent(false);
    let renderer = Renderer::plain();
    assert_eq(expected, renderer.render(input).to_string());
}

#[test]
fn test_only_source() {
    let input = Level::Error
        .title("")
        .snippet(Snippet::source("").origin("file.rs"));
    let expected = str![[r#"
error
--> file.rs
 |
 |"#]]
    .indent(false);
    let renderer = Renderer::plain();
    assert_eq(expected, renderer.render(input).to_string());
}

#[test]
fn test_anon_lines() {
    let source = "This is an example\nof content lines\n\nabc";
    let input = Level::Error
        .title("")
        .snippet(Snippet::source(source).line_start(56));
    let expected = str![[r#"
error
   |
LL | This is an example
LL | of content lines
LL | 
LL | abc
   |"#]]
    .indent(false);
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_eq(expected, renderer.render(input).to_string());
}
