use annotate_snippets::{Label, Level, Renderer, Snippet};

#[test]
fn test_i_29() {
    let snippets = Level::Error.title("oops").snippet(
        Snippet::new("First line\r\nSecond oops line")
            .origin("<current file>")
            .annotation(Label::error("oops").span(19..23))
            .fold(true),
    );
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
    let snippets = Level::Error.title("").snippet(
        Snippet::new("こんにちは、世界")
            .origin("<current file>")
            .annotation(Label::error("world").span(12..16)),
    );

    let expected = r#"error
 --> <current file>:1:7
  |
1 | こんにちは、世界
  |             ^^^^ world
  |"#;

    let renderer = Renderer::plain();
    assert_eq!(renderer.render(snippets).to_string(), expected);
}

#[test]
fn test_point_to_double_width_characters_across_lines() {
    let snippets = Level::Error.title("").snippet(
        Snippet::new("おはよう\nございます")
            .origin("<current file>")
            .annotation(Label::error("Good morning").span(4..15)),
    );

    let expected = r#"error
 --> <current file>:1:3
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
    let snippets = Level::Error.title("").snippet(
        Snippet::new("お寿司\n食べたい🍣")
            .origin("<current file>")
            .annotation(Label::error("Sushi1").span(0..6))
            .annotation(Label::note("Sushi2").span(11..15)),
    );

    let expected = r#"error
 --> <current file>:1:1
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
    let snippets = Level::Error.title("").snippet(
        Snippet::new("こんにちは、新しいWorld！")
            .origin("<current file>")
            .annotation(Label::error("New world").span(12..23)),
    );

    let expected = r#"error
 --> <current file>:1:7
  |
1 | こんにちは、新しいWorld！
  |             ^^^^^^^^^^^ New world
  |"#;

    let renderer = Renderer::plain();
    assert_eq!(renderer.render(snippets).to_string(), expected);
}
