use annotate_snippets::{
    Annotation, AnnotationKind, Group, Level, Padding, Patch, Renderer, Snippet,
};

use annotate_snippets::renderer::OutputTheme;
use snapbox::{assert_data_eq, str};

#[test]
fn test_i_29() {
    let snippets = &[Group::with_title(Level::ERROR.title("oops")).element(
        Snippet::source("First line\r\nSecond oops line")
            .path("<current file>")
            .annotation(AnnotationKind::Primary.span(19..23).label("oops")),
    )];
    let expected = str![[r#"
error: oops
 --> <current file>:2:8
  |
2 | Second oops line
  |        ^^^^ oops
"#]];

    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(snippets), expected);
}

#[test]
fn test_point_to_double_width_characters() {
    let snippets = &[Group::with_title(Level::ERROR.title("")).element(
        Snippet::source("こんにちは、世界")
            .path("<current file>")
            .annotation(AnnotationKind::Primary.span(18..24).label("world")),
    )];

    let expected = str![[r#"
error: 
 --> <current file>:1:7
  |
1 | こんにちは、世界
  |             ^^^^ world
"#]];

    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(snippets), expected);
}

#[test]
fn test_point_to_double_width_characters_across_lines() {
    let snippets = &[Group::with_title(Level::ERROR.title("")).element(
        Snippet::source("おはよう\nございます")
            .path("<current file>")
            .annotation(AnnotationKind::Primary.span(6..22).label("Good morning")),
    )];

    let expected = str![[r#"
error: 
 --> <current file>:1:3
  |
1 |   おはよう
  |  _____^
2 | | ございます
  | |______^ Good morning
"#]];

    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(snippets), expected);
}

#[test]
fn test_point_to_double_width_characters_multiple() {
    let snippets = &[Group::with_title(Level::ERROR.title("")).element(
        Snippet::source("お寿司\n食べたい🍣")
            .path("<current file>")
            .annotation(AnnotationKind::Primary.span(0..9).label("Sushi1"))
            .annotation(AnnotationKind::Context.span(16..22).label("Sushi2")),
    )];

    let expected = str![[r#"
error: 
 --> <current file>:1:1
  |
1 | お寿司
  | ^^^^^^ Sushi1
2 | 食べたい🍣
  |     ---- Sushi2
"#]];

    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(snippets), expected);
}

#[test]
fn test_point_to_double_width_characters_mixed() {
    let snippets = &[Group::with_title(Level::ERROR.title("")).element(
        Snippet::source("こんにちは、新しいWorld！")
            .path("<current file>")
            .annotation(AnnotationKind::Primary.span(18..32).label("New world")),
    )];

    let expected = str![[r#"
error: 
 --> <current file>:1:7
  |
1 | こんにちは、新しいWorld！
  |             ^^^^^^^^^^^ New world
"#]];

    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(snippets), expected);
}

#[test]
fn test_format_title() {
    let input = &[Group::with_title(
        Level::ERROR.title("This is a title").id("E0001"),
    )];

    let expected = str![r#"error[E0001]: This is a title"#];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn test_format_snippet_only() {
    let source = "This is line 1\nThis is line 2";
    let input = &[Group::with_title(Level::ERROR.title("")).element(
        Snippet::<Annotation<'_>>::source(source)
            .line_start(5402)
            .fold(false),
    )];

    let expected = str![[r#"
error: 
     |
5402 | This is line 1
5403 | This is line 2
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn test_format_snippets_continuation() {
    let src_0 = "This is slice 1";
    let src_1 = "This is slice 2";
    let input = &[Group::with_title(Level::ERROR.title(""))
        .element(
            Snippet::<Annotation<'_>>::source(src_0)
                .line_start(5402)
                .path("file1.rs")
                .fold(false),
        )
        .element(
            Snippet::<Annotation<'_>>::source(src_1)
                .line_start(2)
                .path("file2.rs")
                .fold(false),
        )];
    let expected = str![[r#"
error: 
    --> file1.rs
     |
5402 | This is slice 1
     |
    ::: file2.rs:2
     |
   2 | This is slice 2
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn test_format_snippet_annotation_standalone() {
    let line_1 = "This is line 1";
    let line_2 = "This is line 2";
    let source = [line_1, line_2].join("\n");
    // In line 2
    let range = 22..24;
    let input = &[Group::with_title(Level::ERROR.title("")).element(
        Snippet::source(&source)
            .line_start(5402)
            .fold(false)
            .annotation(
                AnnotationKind::Context
                    .span(range.clone())
                    .label("Test annotation"),
            ),
    )];
    let expected = str![[r#"
error: 
     |
5402 | This is line 1
5403 | This is line 2
     |        -- Test annotation
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn test_format_footer_title() {
    let input = &[Group::with_title(Level::ERROR.title(""))
        .element(Level::ERROR.message("This __is__ a title"))];
    let expected = str![[r#"
error: 
  |
  = error: This __is__ a title
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
#[should_panic]
fn test_i26() {
    let source = "short";
    let label = "label";
    let input = &[Group::with_title(Level::ERROR.title("")).element(
        Snippet::source(source).line_start(0).annotation(
            AnnotationKind::Primary
                .span(0..source.len() + 2)
                .label(label),
        ),
    )];
    let renderer = Renderer::plain();
    let _ = renderer.render(input);
}

#[test]
fn test_source_content() {
    let source = "This is an example\nof content lines";
    let input = &[Group::with_title(Level::ERROR.title("")).element(
        Snippet::<Annotation<'_>>::source(source)
            .line_start(56)
            .fold(false),
    )];
    let expected = str![[r#"
error: 
   |
56 | This is an example
57 | of content lines
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn test_source_annotation_standalone_singleline() {
    let source = "tests";
    let input = &[Group::with_title(Level::ERROR.title("")).element(
        Snippet::source(source)
            .line_start(1)
            .annotation(AnnotationKind::Context.span(0..5).label("Example string")),
    )];
    let expected = str![[r#"
error: 
  |
1 | tests
  | ----- Example string
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn test_source_annotation_standalone_multiline() {
    let source = "tests";
    let input = &[Group::with_title(Level::ERROR.title("")).element(
        Snippet::source(source)
            .line_start(1)
            .annotation(AnnotationKind::Context.span(0..5).label("Example string"))
            .annotation(AnnotationKind::Context.span(0..5).label("Second line")),
    )];
    let expected = str![[r#"
error: 
  |
1 | tests
  | -----
  | |
  | Example string
  | Second line
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn test_only_source() {
    let input = &[Group::with_title(Level::ERROR.title("")).element(
        Snippet::<Annotation<'_>>::source("")
            .path("file.rs")
            .fold(false),
    )];
    let expected = str![[r#"
error: 
 --> file.rs
  |
1 |
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn test_anon_lines() {
    let source = "This is an example\nof content lines\n\nabc";
    let input = &[Group::with_title(Level::ERROR.title("")).element(
        Snippet::<Annotation<'_>>::source(source)
            .line_start(56)
            .fold(false),
    )];
    let expected = str![[r#"
error: 
   |
LL | This is an example
LL | of content lines
LL |
LL | abc
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn issue_130() {
    let input = &[Group::with_title(Level::ERROR.title("dummy")).element(
        Snippet::source("foo\nbar\nbaz")
            .path("file/path")
            .line_start(3)
            .annotation(AnnotationKind::Primary.span(4..11)),
        // bar\nbaz
    )];

    let expected = str![[r#"
error: dummy
 --> file/path:4:1
  |
4 | / bar
5 | | baz
  | |___^
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn unterminated_string_multiline() {
    let source = "\
a\"
// ...
";
    let input = &[Group::with_title(Level::ERROR.title("")).element(
        Snippet::source(source)
            .path("file/path")
            .line_start(3)
            .annotation(AnnotationKind::Primary.span(0..10)),
        // 1..10 works
    )];
    let expected = str![[r#"
error: 
 --> file/path:3:1
  |
3 | / a"
4 | | // ...
  | |_______^
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(false);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn char_and_nl_annotate_char() {
    let source = "a\r\nb";
    let input = &[Group::with_title(Level::ERROR.title("")).element(
        Snippet::source(source)
            .path("file/path")
            .line_start(3)
            .fold(false)
            .annotation(AnnotationKind::Primary.span(0..2)),
        // a\r
    )];
    let expected = str![[r#"
error: 
 --> file/path:3:1
  |
3 | a
  | ^
4 | b
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(false);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn char_eol_annotate_char() {
    let source = "a\r\nb";
    let input = &[Group::with_title(Level::ERROR.title("")).element(
        Snippet::source(source)
            .path("file/path")
            .line_start(3)
            .annotation(AnnotationKind::Primary.span(0..3)),
        // a\r\n
    )];
    let expected = str![[r#"
error: 
 --> file/path:3:1
  |
3 | / a
4 | | b
  | |_^
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(false);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn char_eol_annotate_char_double_width() {
    let snippets = &[Group::with_title(Level::ERROR.title("")).element(
        Snippet::source("こん\r\nにちは\r\n世界")
            .path("<current file>")
            .fold(false)
            .annotation(AnnotationKind::Primary.span(3..8)),
        // ん\r\n
    )];

    let expected = str![[r#"
error: 
 --> <current file>:1:2
  |
1 |   こん
  |  ___^
2 | | にちは
  | |_^
3 |   世界
"#]];

    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(snippets), expected);
}

#[test]
fn annotate_eol() {
    let source = "a\r\nb";
    let input = &[Group::with_title(Level::ERROR.title("")).element(
        Snippet::source(source)
            .path("file/path")
            .line_start(3)
            .fold(false)
            .annotation(AnnotationKind::Primary.span(1..2)),
        // \r
    )];
    let expected = str![[r#"
error: 
 --> file/path:3:2
  |
3 | a
  |  ^
4 | b
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(false);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn annotate_eol2() {
    let source = "a\r\nb";
    let input = &[Group::with_title(Level::ERROR.title("")).element(
        Snippet::source(source)
            .path("file/path")
            .line_start(3)
            .annotation(AnnotationKind::Primary.span(1..3)),
        // \r\n
    )];
    let expected = str![[r#"
error: 
 --> file/path:3:2
  |
3 |   a
  |  __^
4 | | b
  | |_^
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(false);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn annotate_eol3() {
    let source = "a\r\nb";
    let input = &[Group::with_title(Level::ERROR.title("")).element(
        Snippet::source(source)
            .path("file/path")
            .line_start(3)
            .annotation(AnnotationKind::Primary.span(2..3)),
        // \n
    )];
    let expected = str![[r#"
error: 
 --> file/path:3:3
  |
3 |   a
  |  __^
4 | | b
  | |_^
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(false);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn annotate_eol4() {
    let source = "a\r\nb";
    let input = &[Group::with_title(Level::ERROR.title("")).element(
        Snippet::source(source)
            .path("file/path")
            .line_start(3)
            .fold(false)
            .annotation(AnnotationKind::Primary.span(2..2)),
        // \n
    )];
    let expected = str![[r#"
error: 
 --> file/path:3:3
  |
3 | a
  |  ^
4 | b
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(false);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn annotate_eol_double_width() {
    let snippets = &[Group::with_title(Level::ERROR.title("")).element(
        Snippet::source("こん\r\nにちは\r\n世界")
            .path("<current file>")
            .fold(false)
            .annotation(AnnotationKind::Primary.span(7..8)),
        // \n
    )];

    let expected = str![[r#"
error: 
 --> <current file>:1:4
  |
1 |   こん
  |  _____^
2 | | にちは
  | |_^
3 |   世界
"#]];

    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(snippets), expected);
}

#[test]
fn multiline_eol_start() {
    let source = "a\r\nb";
    let input = &[Group::with_title(Level::ERROR.title("")).element(
        Snippet::source(source)
            .path("file/path")
            .line_start(3)
            .annotation(AnnotationKind::Primary.span(1..4)),
        // \r\nb
    )];
    let expected = str![[r#"
error: 
 --> file/path:3:2
  |
3 |   a
  |  __^
4 | | b
  | |_^
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(false);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn multiline_eol_start2() {
    let source = "a\r\nb";
    let input = &[Group::with_title(Level::ERROR.title("")).element(
        Snippet::source(source)
            .path("file/path")
            .line_start(3)
            .annotation(AnnotationKind::Primary.span(2..4)),
        // \nb
    )];
    let expected = str![[r#"
error: 
 --> file/path:3:3
  |
3 |   a
  |  __^
4 | | b
  | |_^
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(false);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn multiline_eol_start3() {
    let source = "a\nb";
    let input = &[Group::with_title(Level::ERROR.title("")).element(
        Snippet::source(source)
            .path("file/path")
            .line_start(3)
            .annotation(AnnotationKind::Primary.span(1..3)),
        // \nb
    )];
    let expected = str![[r#"
error: 
 --> file/path:3:2
  |
3 |   a
  |  __^
4 | | b
  | |_^
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(false);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn multiline_eol_start_double_width() {
    let snippets = &[Group::with_title(Level::ERROR.title("")).element(
        Snippet::source("こん\r\nにちは\r\n世界")
            .path("<current file>")
            .fold(false)
            .annotation(AnnotationKind::Primary.span(7..11)),
        // \r\nに
    )];

    let expected = str![[r#"
error: 
 --> <current file>:1:4
  |
1 |   こん
  |  _____^
2 | | にちは
  | |__^
3 |   世界
"#]];

    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(snippets), expected);
}

#[test]
fn multiline_eol_start_eol_end() {
    let source = "a\nb\nc";
    let input = &[Group::with_title(Level::ERROR.title("")).element(
        Snippet::source(source)
            .path("file/path")
            .line_start(3)
            .annotation(AnnotationKind::Primary.span(1..4)),
        // \nb\n
    )];
    let expected = str![[r#"
error: 
 --> file/path:3:2
  |
3 |   a
  |  __^
4 | | b
5 | | c
  | |_^
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(false);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn multiline_eol_start_eol_end2() {
    let source = "a\r\nb\r\nc";
    let input = &[Group::with_title(Level::ERROR.title("")).element(
        Snippet::source(source)
            .path("file/path")
            .line_start(3)
            .fold(false)
            .annotation(AnnotationKind::Primary.span(2..5)),
        // \nb\r
    )];
    let expected = str![[r#"
error: 
 --> file/path:3:3
  |
3 |   a
  |  __^
4 | | b
  | |__^
5 |   c
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(false);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn multiline_eol_start_eol_end3() {
    let source = "a\r\nb\r\nc";
    let input = &[Group::with_title(Level::ERROR.title("")).element(
        Snippet::source(source)
            .path("file/path")
            .line_start(3)
            .annotation(AnnotationKind::Primary.span(2..6)),
        // \nb\r\n
    )];
    let expected = str![[r#"
error: 
 --> file/path:3:3
  |
3 |   a
  |  __^
4 | | b
5 | | c
  | |_^
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(false);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn multiline_eol_start_eof_end() {
    let source = "a\r\nb";
    let input = &[Group::with_title(Level::ERROR.title("")).element(
        Snippet::source(source)
            .path("file/path")
            .line_start(3)
            .annotation(AnnotationKind::Primary.span(1..5)),
        // \r\nb(EOF)
    )];
    let expected = str![[r#"
error: 
 --> file/path:3:2
  |
3 |   a
  |  __^
4 | | b
  | |__^
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(false);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn multiline_eol_start_eof_end_double_width() {
    let source = "ん\r\nに";
    let input = &[Group::with_title(Level::ERROR.title("")).element(
        Snippet::source(source)
            .path("file/path")
            .line_start(3)
            .annotation(AnnotationKind::Primary.span(3..9)),
        // \r\nに(EOF)
    )];
    let expected = str![[r#"
error: 
 --> file/path:3:2
  |
3 |   ん
  |  ___^
4 | | に
  | |___^
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(false);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn two_single_line_same_line() {
    let source = r#"bar = { version = "0.1.0", optional = true }"#;
    let input = &[
        Group::with_title(Level::ERROR.title("unused optional dependency")).element(
            Snippet::source(source)
                .path("Cargo.toml")
                .line_start(4)
                .annotation(
                    AnnotationKind::Primary
                        .span(0..3)
                        .label("I need this to be really long so I can test overlaps"),
                )
                .annotation(
                    AnnotationKind::Context
                        .span(27..42)
                        .label("This should also be long but not too long"),
                ),
        ),
    ];
    let expected = str![[r#"
error: unused optional dependency
 --> Cargo.toml:4:1
  |
4 | bar = { version = "0.1.0", optional = true }
  | ^^^                        --------------- This should also be long but not too long
  | |
  | I need this to be really long so I can test overlaps
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(false);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn multi_and_single() {
    let source = r#"bar = { version = "0.1.0", optional = true }
this is another line
so is this
bar = { version = "0.1.0", optional = true }
"#;
    let input = &[
        Group::with_title(Level::ERROR.title("unused optional dependency")).element(
            Snippet::source(source)
                .line_start(4)
                .annotation(
                    AnnotationKind::Primary
                        .span(41..119)
                        .label("I need this to be really long so I can test overlaps"),
                )
                .annotation(
                    AnnotationKind::Context
                        .span(27..42)
                        .label("This should also be long but not too long"),
                ),
        ),
    ];
    let expected = str![[r#"
error: unused optional dependency
  |
4 |   bar = { version = "0.1.0", optional = true }
  |  ____________________________--------------^
  | |                            |
  | |                            This should also be long but not too long
5 | | this is another line
6 | | so is this
7 | | bar = { version = "0.1.0", optional = true }
  | |__________________________________________^ I need this to be really long so I can test overlaps
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn two_multi_and_single() {
    let source = r#"bar = { version = "0.1.0", optional = true }
this is another line
so is this
bar = { version = "0.1.0", optional = true }
"#;
    let input = &[
        Group::with_title(Level::ERROR.title("unused optional dependency")).element(
            Snippet::source(source)
                .line_start(4)
                .annotation(
                    AnnotationKind::Primary
                        .span(41..119)
                        .label("I need this to be really long so I can test overlaps"),
                )
                .annotation(
                    AnnotationKind::Primary
                        .span(8..102)
                        .label("I need this to be really long so I can test overlaps"),
                )
                .annotation(
                    AnnotationKind::Context
                        .span(27..42)
                        .label("This should also be long but not too long"),
                ),
        ),
    ];
    let expected = str![[r#"
error: unused optional dependency
  |
4 |    bar = { version = "0.1.0", optional = true }
  |  __________^__________________--------------^
  | |          |                  |
  | | _________|                  This should also be long but not too long
  | ||
5 | || this is another line
6 | || so is this
7 | || bar = { version = "0.1.0", optional = true }
  | ||_________________________^________________^ I need this to be really long so I can test overlaps
  |  |_________________________|
  |                            I need this to be really long so I can test overlaps
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn three_multi_and_single() {
    let source = r#"bar = { version = "0.1.0", optional = true }
this is another line
so is this
bar = { version = "0.1.0", optional = true }
this is another line
"#;
    let input = &[
        Group::with_title(Level::ERROR.title("unused optional dependency")).element(
            Snippet::source(source)
                .line_start(4)
                .annotation(
                    AnnotationKind::Primary
                        .span(41..119)
                        .label("I need this to be really long so I can test overlaps"),
                )
                .annotation(
                    AnnotationKind::Primary
                        .span(8..102)
                        .label("I need this to be really long so I can test overlaps"),
                )
                .annotation(
                    AnnotationKind::Primary
                        .span(48..126)
                        .label("I need this to be really long so I can test overlaps"),
                )
                .annotation(
                    AnnotationKind::Context
                        .span(27..42)
                        .label("This should also be long but not too long"),
                ),
        ),
    ];
    let expected = str![[r#"
error: unused optional dependency
  |
4 |     bar = { version = "0.1.0", optional = true }
  |  ___________^__________________--------------^
  | |           |                  |
  | | __________|                  This should also be long but not too long
  | ||
5 | ||  this is another line
  | || ____^
6 | ||| so is this
7 | ||| bar = { version = "0.1.0", optional = true }
  | |||_________________________^________________^ I need this to be really long so I can test overlaps
  |  ||_________________________|
  |   |                         I need this to be really long so I can test overlaps
8 |   | this is another line
  |   |____^ I need this to be really long so I can test overlaps
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn origin_correct_start_line() {
    let source = "aaa\nbbb\nccc\nddd\n";
    let input = &[Group::with_title(Level::ERROR.title("title")).element(
        Snippet::source(source)
            .path("origin.txt")
            .fold(false)
            .annotation(AnnotationKind::Primary.span(8..8 + 3).label("annotation")),
    )];

    let expected = str![[r#"
error: title
 --> origin.txt:3:1
  |
1 | aaa
2 | bbb
3 | ccc
  | ^^^ annotation
4 | ddd
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn origin_correct_mid_line() {
    let source = "aaa\nbbb\nccc\nddd\n";
    let input = &[Group::with_title(Level::ERROR.title("title")).element(
        Snippet::source(source)
            .path("origin.txt")
            .fold(false)
            .annotation(
                AnnotationKind::Primary
                    .span(8 + 1..8 + 3)
                    .label("annotation"),
            ),
    )];

    let expected = str![[r#"
error: title
 --> origin.txt:3:2
  |
1 | aaa
2 | bbb
3 | ccc
  |  ^^ annotation
4 | ddd
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn two_suggestions_same_span() {
    let source = r#"    A.foo();"#;
    let input_new = &[
        Group::with_title(
            Level::ERROR
                .title("expected value, found enum `A`")
                .id("E0423"),
        )
        .element(Snippet::source(source).annotation(AnnotationKind::Primary.span(4..5))),
        Group::with_title(
            Level::HELP.title("you might have meant to use one of the following enum variants"),
        )
        .element(Snippet::source(source).patch(Patch::new(4..5, "(A::Tuple())")))
        .element(Snippet::source(source).patch(Patch::new(4..5, "A::Unit"))),
    ];

    let expected = str![[r#"
error[E0423]: expected value, found enum `A`
   |
LL |     A.foo();
   |     ^
   |
help: you might have meant to use one of the following enum variants
   |
LL -     A.foo();
LL +     (A::Tuple()).foo();
   |
LL |     A::Unit.foo();
   |      ++++++
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input_new), expected);
}

#[test]
fn two_suggestions_same_span2() {
    let source = r#"
mod banana {
    pub struct Chaenomeles;

    pub trait Apple {
        fn pick(&self) {}
    }
    impl Apple for Chaenomeles {}

    pub trait Peach {
        fn pick(&self, a: &mut ()) {}
    }
    impl<Mango: Peach> Peach for Box<Mango> {}
    impl Peach for Chaenomeles {}
}

fn main() {
    banana::Chaenomeles.pick()
}"#;
    let input_new =
        &[Group::with_title(Level::ERROR
            .title("no method named `pick` found for struct `Chaenomeles` in the current scope")
            .id("E0599")).element(
                    Snippet::source(source)
                        .line_start(1)

                        .annotation(
                            AnnotationKind::Context
                                .span(18..40)
                                .label("method `pick` not found for this struct"),
                        )
                        .annotation(
                            AnnotationKind::Primary
                                .span(318..322)
                                .label("method not found in `Chaenomeles`"),
                        ),
                ),
                Group::with_title(Level::HELP.title(
                        "the following traits which provide `pick` are implemented but not in scope; perhaps you want to import one of them",
                    ))
                    .element(
                        Snippet::source(source)

                            .patch(Patch::new(1..1, "use banana::Apple;\n")),
                    )
                    .element(
                        Snippet::source(source)

                            .patch(Patch::new(1..1, "use banana::Peach;\n")),
                   )];
    let expected = str![[r#"
error[E0599]: no method named `pick` found for struct `Chaenomeles` in the current scope
   |
LL |     pub struct Chaenomeles;
   |     ---------------------- method `pick` not found for this struct
...
LL |     banana::Chaenomeles.pick()
   |                         ^^^^ method not found in `Chaenomeles`
   |
help: the following traits which provide `pick` are implemented but not in scope; perhaps you want to import one of them
   |
LL + use banana::Apple;
   |
LL + use banana::Peach;
   |
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input_new), expected);
}

#[test]
fn single_line_non_overlapping_suggestions() {
    let source = r#"    A.foo();"#;

    let input_new = &[
        Group::with_title(
            Level::ERROR
                .title("expected value, found enum `A`")
                .id("E0423"),
        )
        .element(
            Snippet::source(source)
                .line_start(1)
                .annotation(AnnotationKind::Primary.span(4..5)),
        ),
        Group::with_title(Level::HELP.title("make these changes and things will work")).element(
            Snippet::source(source)
                .patch(Patch::new(4..5, "(A::Tuple())"))
                .patch(Patch::new(6..9, "bar")),
        ),
    ];

    let expected = str![[r#"
error[E0423]: expected value, found enum `A`
   |
LL |     A.foo();
   |     ^
   |
help: make these changes and things will work
   |
LL -     A.foo();
LL +     (A::Tuple()).bar();
   |
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input_new), expected);
}

#[test]
fn single_line_non_overlapping_suggestions2() {
    let source = r#"    ThisIsVeryLong.foo();"#;
    let input_new = &[
        Group::with_title(Level::ERROR.title("Found `ThisIsVeryLong`").id("E0423")).element(
            Snippet::source(source)
                .line_start(1)
                .annotation(AnnotationKind::Primary.span(4..18)),
        ),
        Group::with_title(Level::HELP.title("make these changes and things will work")).element(
            Snippet::source(source)
                .patch(Patch::new(4..18, "(A::Tuple())"))
                .patch(Patch::new(19..22, "bar")),
        ),
    ];

    let expected = str![[r#"
error[E0423]: Found `ThisIsVeryLong`
   |
LL |     ThisIsVeryLong.foo();
   |     ^^^^^^^^^^^^^^
   |
help: make these changes and things will work
   |
LL -     ThisIsVeryLong.foo();
LL +     (A::Tuple()).bar();
   |
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input_new), expected);
}

#[test]
fn multiple_replacements() {
    let source = r#"
    let y = || {
        self.bar();
    };
    self.qux();
    y();
"#;

    let input_new = &[
        Group::with_title(
            Level::ERROR
                .title("cannot borrow `*self` as mutable because it is also borrowed as immutable")
                .id("E0502"),
        )
        .element(
            Snippet::source(source)
                .line_start(1)
                .annotation(
                    AnnotationKind::Primary
                        .span(49..59)
                        .label("mutable borrow occurs here"),
                )
                .annotation(
                    AnnotationKind::Primary
                        .span(13..15)
                        .label("immutable borrow occurs here"),
                )
                .annotation(
                    AnnotationKind::Primary
                        .span(26..30)
                        .label("first borrow occurs due to use of `*self` in closure"),
                )
                .annotation(
                    AnnotationKind::Primary
                        .span(65..66)
                        .label("immutable borrow later used here"),
                ),
        ),
        Group::with_title(
            Level::HELP.title("try explicitly pass `&Self` into the Closure as an argument"),
        )
        .element(
            Snippet::source(source)
                .patch(Patch::new(14..14, "this: &Self"))
                .patch(Patch::new(26..30, "this"))
                .patch(Patch::new(66..68, "(self)")),
        ),
    ];
    let expected = str![[r#"
error[E0502]: cannot borrow `*self` as mutable because it is also borrowed as immutable
   |
LL |     let y = || {
   |             ^^ immutable borrow occurs here
LL |         self.bar();
   |         ^^^^ first borrow occurs due to use of `*self` in closure
LL |     };
LL |     self.qux();
   |     ^^^^^^^^^^ mutable borrow occurs here
LL |     y();
   |     ^ immutable borrow later used here
   |
help: try explicitly pass `&Self` into the Closure as an argument
   |
LL ~     let y = |this: &Self| {
LL ~         this.bar();
LL |     };
LL |     self.qux();
LL ~     y(self);
   |
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input_new), expected);
}

#[test]
fn multiple_replacements2() {
    let source = r#"
fn test1() {
    let mut chars = "Hello".chars();
    for _c in chars.by_ref() {
        chars.next();
    }
}

fn main() {
    test1();
}"#;

    let input_new = &[
        Group::with_title(
            Level::ERROR
                .title("cannot borrow `chars` as mutable more than once at a time")
                .id("E0499"),
        )
        .element(
            Snippet::source(source)
                .line_start(1)
                .annotation(
                    AnnotationKind::Context
                        .span(65..70)
                        .label("first mutable borrow occurs here"),
                )
                .annotation(
                    AnnotationKind::Primary
                        .span(90..95)
                        .label("second mutable borrow occurs here"),
                )
                .annotation(
                    AnnotationKind::Context
                        .span(65..79)
                        .label("first borrow later used here"),
                ),
        ),
        Group::with_title(Level::HELP.title(
            "if you want to call `next` on a iterator within the loop, consider using `while let`",
        ))
        .element(
            Snippet::source(source)
                .patch(Patch::new(
                    55..59,
                    "let iter = chars.by_ref();\n    while let Some(",
                ))
                .patch(Patch::new(61..79, ") = iter.next()"))
                .patch(Patch::new(90..95, "iter")),
        ),
    ];

    let expected = str![[r#"
error[E0499]: cannot borrow `chars` as mutable more than once at a time
   |
LL |     for _c in chars.by_ref() {
   |               --------------
   |               |
   |               first mutable borrow occurs here
   |               first borrow later used here
LL |         chars.next();
   |         ^^^^^ second mutable borrow occurs here
   |
help: if you want to call `next` on a iterator within the loop, consider using `while let`
   |
LL ~     let iter = chars.by_ref();
LL ~     while let Some(_c) = iter.next() {
LL ~         iter.next();
   |
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input_new), expected);
}

#[test]
fn diff_format() {
    let source = r#"
use st::cell::Cell;

mod bar {
    pub fn bar() { bar::baz(); }

    fn baz() {}
}

use bas::bar;

struct Foo {
    bar: st::cell::Cell<bool>
}

fn main() {}"#;

    let input_new = &[
        Group::with_title(
            Level::ERROR
                .title("failed to resolve: use of undeclared crate or module `st`")
                .id("E0433"),
        )
        .element(
            Snippet::source(source).line_start(1).annotation(
                AnnotationKind::Primary
                    .span(122..124)
                    .label("use of undeclared crate or module `st`"),
            ),
        ),
        Group::with_title(Level::HELP.title("there is a crate or module with a similar name"))
            .element(Snippet::source(source).patch(Patch::new(122..124, "std"))),
        Group::with_title(Level::HELP.title("consider importing this module"))
            .element(Snippet::source(source).patch(Patch::new(1..1, "use std::cell;\n"))),
        Group::with_title(Level::HELP.title("if you import `cell`, refer to it directly"))
            .element(Snippet::source(source).patch(Patch::new(122..126, ""))),
    ];
    let expected = str![[r#"
error[E0433]: failed to resolve: use of undeclared crate or module `st`
   |
LL |     bar: st::cell::Cell<bool>
   |          ^^ use of undeclared crate or module `st`
   |
help: there is a crate or module with a similar name
   |
LL |     bar: std::cell::Cell<bool>
   |            +
help: consider importing this module
   |
LL + use std::cell;
   |
help: if you import `cell`, refer to it directly
   |
LL -     bar: st::cell::Cell<bool>
LL +     bar: cell::Cell<bool>
   |
"#]];

    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input_new), expected);
}

#[test]
fn multiline_removal() {
    let source = r#"
struct Wrapper<T>(T);

fn foo<T>(foo: Wrapper<T>)

where
    T
    :
    ?
    Sized
{
    //
}

fn main() {}"#;

    let input_new = &[
        Group::with_title(
            Level::ERROR
                .title("the size for values of type `T` cannot be known at compilation time")
                .id("E0277"),
        )
        .element(
            Snippet::source(source)
                .line_start(1)
                .annotation(
                    AnnotationKind::Primary
                        .span(39..49)
                        .label("doesn't have a size known at compile-time"),
                )
                .annotation(
                    AnnotationKind::Context
                        .span(31..32)
                        .label("this type parameter needs to be `Sized`"),
                ),
        ),
        Group::with_title(
            Level::HELP
                .title("consider removing the `?Sized` bound to make the type parameter `Sized`"),
        )
        .element(Snippet::source(source).patch(Patch::new(52..85, ""))),
    ];
    let expected = str![[r#"
error[E0277]: the size for values of type `T` cannot be known at compilation time
   |
LL | fn foo<T>(foo: Wrapper<T>)
   |        -       ^^^^^^^^^^ doesn't have a size known at compile-time
   |        |
   |        this type parameter needs to be `Sized`
   |
help: consider removing the `?Sized` bound to make the type parameter `Sized`
   |
LL - where
LL -     T
LL -     :
LL -     ?
LL -     Sized
   |
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input_new), expected);
}

#[test]
fn multiline_replacement() {
    let source = r#"
struct Wrapper<T>(T);

fn foo<T>(foo: Wrapper<T>)

and where
    T
    :
    ?
    Sized
{
    //
}

fn main() {}"#;
    let input_new = &[Group::with_title(Level::ERROR
        .title("the size for values of type `T` cannot be known at compilation time")
        .id("E0277")).element(Snippet::source(source)
            .line_start(1)
            .path("$DIR/removal-of-multiline-trait-bound-in-where-clause.rs")

            .annotation(
                AnnotationKind::Primary
                    .span(39..49)
                    .label("doesn't have a size known at compile-time"),
            )
            .annotation(
                AnnotationKind::Context
                    .span(31..32)
                    .label("this type parameter needs to be `Sized`"),
            ))
        ,Group::with_title(
            Level::NOTE
                .title("required by an implicit `Sized` bound in `Wrapper`")
        ).element(
            Snippet::source(source)
                .line_start(1)
                .path("$DIR/removal-of-multiline-trait-bound-in-where-clause.rs")

                .annotation(
                    AnnotationKind::Primary
                        .span(16..17)
                        .label("required by the implicit `Sized` requirement on this type parameter in `Wrapper`"),
                )
        ), Group::with_title(
            Level::HELP
                .title("you could relax the implicit `Sized` bound on `T` if it were used through indirection like `&T` or `Box<T>`")
            )
            .element(
            Snippet::source(source)
                .line_start(1)
                .path("$DIR/removal-of-multiline-trait-bound-in-where-clause.rs")

                .annotation(
                    AnnotationKind::Primary
                        .span(16..17)
                        .label("this could be changed to `T: ?Sized`..."),
                )
                .annotation(
                    AnnotationKind::Context
                        .span(19..20)
                        .label("...if indirection were used here: `Box<T>`"),
                )

        ),Group::with_title(
            Level::HELP
                .title("consider removing the `?Sized` bound to make the type parameter `Sized`")
        ).element(
            Snippet::source(source)

                .patch(Patch::new(56..89, ""))
                .patch(Patch::new(89..89, "+ Send"))
                ,
        )];
    let expected = str![[r#"
error[E0277]: the size for values of type `T` cannot be known at compilation time
  --> $DIR/removal-of-multiline-trait-bound-in-where-clause.rs:4:16
   |
LL | fn foo<T>(foo: Wrapper<T>)
   |        -       ^^^^^^^^^^ doesn't have a size known at compile-time
   |        |
   |        this type parameter needs to be `Sized`
   |
note: required by an implicit `Sized` bound in `Wrapper`
  --> $DIR/removal-of-multiline-trait-bound-in-where-clause.rs:2:16
   |
LL | struct Wrapper<T>(T);
   |                ^ required by the implicit `Sized` requirement on this type parameter in `Wrapper`
help: you could relax the implicit `Sized` bound on `T` if it were used through indirection like `&T` or `Box<T>`
  --> $DIR/removal-of-multiline-trait-bound-in-where-clause.rs:2:16
   |
LL | struct Wrapper<T>(T);
   |                ^  - ...if indirection were used here: `Box<T>`
   |                |
   |                this could be changed to `T: ?Sized`...
help: consider removing the `?Sized` bound to make the type parameter `Sized`
   |
LL - and where
LL -     T
LL -     :
LL -     ?
LL -     Sized
LL + and + Send
   |
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input_new), expected);
}

#[test]
fn multiline_removal2() {
    let source = r#"
cargo
fuzzy
pizza
jumps
crazy
quack
zappy
"#;

    let input_new = &[
        Group::with_title(
            Level::ERROR
                .title("the size for values of type `T` cannot be known at compilation time")
                .id("E0277"),
        ),
        // We need an empty group here to ensure the HELP line is rendered correctly
        Group::with_title(
            Level::HELP
                .title("consider removing the `?Sized` bound to make the type parameter `Sized`"),
        )
        .element(
            Snippet::source(source)
                .line_start(7)
                .patch(Patch::new(3..21, ""))
                .patch(Patch::new(22..40, "")),
        ),
    ];
    let expected = str![[r#"
error[E0277]: the size for values of type `T` cannot be known at compilation time
   |
help: consider removing the `?Sized` bound to make the type parameter `Sized`
   |
 8 - cargo
 9 - fuzzy
10 - pizza
11 - jumps
 8 + campy
   |
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input_new), expected);
}

#[test]
fn e0271() {
    let source = r#"
trait Future {
    type Error;
}

impl<T, E> Future for Result<T, E> {
    type Error = E;
}

impl<T> Future for Option<T> {
    type Error = ();
}

struct Foo;

fn foo() -> Box<dyn Future<Error=Foo>> {
    Box::new(
        Ok::<_, ()>(
            Err::<(), _>(
                Ok::<_, ()>(
                    Err::<(), _>(
                        Ok::<_, ()>(
                            Err::<(), _>(Some(5))
                        )
                    )
                )
            )
        )
    )
}
fn main() {
}
"#;

    let input_new = &[Group::with_title(Level::ERROR
        .title("type mismatch resolving `<Result<Result<(), Result<Result<(), Result<Result<(), Option<{integer}>>, ...>>, ...>>, ...> as Future>::Error == Foo`")
        .id("E0271")).element(Snippet::source(source)
            .line_start(4)
            .path("$DIR/E0271.rs")

            .annotation(
                AnnotationKind::Primary
                    .span(208..510)
                    .label("type mismatch resolving `<Result<Result<(), Result<Result<(), ...>, ...>>, ...> as Future>::Error == Foo`"),
            )),Group::with_title(
            Level::NOTE.title("expected this to be `Foo`")
        ).element(
            Snippet::source(source)
                .line_start(4)
                .path("$DIR/E0271.rs")

                .annotation(AnnotationKind::Primary.span(89..90))
        ).element(
            Level::NOTE
                .title("required for the cast from `Box<Result<Result<(), Result<Result<(), Result<Result<(), Option<{integer}>>, ()>>, ()>>, ()>>` to `Box<(dyn Future<Error = Foo> + 'static)>`")
                ,
        )];

    let expected = str![[r#"
error[E0271]: type mismatch resolving `<Result<Result<(), Result<Result<(), Result<Result<(), Option<{integer}>>, ...>>, ...>>, ...> as Future>::Error == Foo`
   ╭▸ $DIR/E0271.rs:20:5
   │
LL │ ┏     Box::new(
LL │ ┃         Ok::<_, ()>(
LL │ ┃             Err::<(), _>(
LL │ ┃                 Ok::<_, ()>(
   ‡ ┃
LL │ ┃     )
   │ ┗━━━━━┛ type mismatch resolving `<Result<Result<(), Result<Result<(), ...>, ...>>, ...> as Future>::Error == Foo`
   ╰╴
note: expected this to be `Foo`
   ╭▸ $DIR/E0271.rs:10:18
   │
LL │     type Error = E;
   │                  ━
   ╰ note: required for the cast from `Box<Result<Result<(), Result<Result<(), Result<Result<(), Option<{integer}>>, ()>>, ()>>, ()>>` to `Box<(dyn Future<Error = Foo> + 'static)>`
"#]];
    let renderer = Renderer::plain()
        .term_width(40)
        .theme(OutputTheme::Unicode)
        .anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input_new), expected);
}

#[test]
fn e0271_2() {
    let source = r#"
trait Future {
    type Error;
}

impl<T, E> Future for Result<T, E> {
    type Error = E;
}

impl<T> Future for Option<T> {
    type Error = ();
}

struct Foo;

fn foo() -> Box<dyn Future<Error=Foo>> {
    Box::new(
        Ok::<_, ()>(
            Err::<(), _>(
                Ok::<_, ()>(
                    Err::<(), _>(
                        Ok::<_, ()>(
                            Err::<(), _>(Some(5))
                        )
                    )
                )
            )
        )
    )
}
fn main() {
}
"#;

    let input_new = &[Group::with_title(Level::ERROR
        .title("type mismatch resolving `<Result<Result<(), Result<Result<(), Result<Result<(), Option<{integer}>>, ...>>, ...>>, ...> as Future>::Error == Foo`")
        .id("E0271")).element(Snippet::source(source)
            .line_start(4)
            .path("$DIR/E0271.rs")

            .annotation(
                AnnotationKind::Primary
                    .span(208..510)
                    .label("type mismatch resolving `<Result<Result<(), Result<Result<(), ...>, ...>>, ...> as Future>::Error == Foo`"),
            )),Group::with_title(
            Level::NOTE.title("expected this to be `Foo`")
        ).element(
            Snippet::source(source)
                .line_start(4)
                .path("$DIR/E0271.rs")

                .annotation(AnnotationKind::Primary.span(89..90))
        ).element(
            Level::NOTE
                .title("required for the cast from `Box<Result<Result<(), Result<Result<(), Result<Result<(), Option<{integer}>>, ()>>, ()>>, ()>>` to `Box<(dyn Future<Error = Foo> + 'static)>`")
        ).element(
            Level::NOTE.title("a second note"),
        )];

    let expected = str![[r#"
error[E0271]: type mismatch resolving `<Result<Result<(), Result<Result<(), Result<Result<(), Option<{integer}>>, ...>>, ...>>, ...> as Future>::Error == Foo`
   ╭▸ $DIR/E0271.rs:20:5
   │
LL │ ┏     Box::new(
LL │ ┃         Ok::<_, ()>(
LL │ ┃             Err::<(), _>(
LL │ ┃                 Ok::<_, ()>(
   ‡ ┃
LL │ ┃     )
   │ ┗━━━━━┛ type mismatch resolving `<Result<Result<(), Result<Result<(), ...>, ...>>, ...> as Future>::Error == Foo`
   ╰╴
note: expected this to be `Foo`
   ╭▸ $DIR/E0271.rs:10:18
   │
LL │     type Error = E;
   │                  ━
   ├ note: required for the cast from `Box<Result<Result<(), Result<Result<(), Result<Result<(), Option<{integer}>>, ()>>, ()>>, ()>>` to `Box<(dyn Future<Error = Foo> + 'static)>`
   ╰ note: a second note
"#]];
    let renderer = Renderer::plain()
        .term_width(40)
        .theme(OutputTheme::Unicode)
        .anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input_new), expected);
}

#[test]
fn long_e0308() {
    let source = r#"
mod a {
    // Force the "short path for unique types" machinery to trip up
    pub struct Atype;
    pub struct Btype;
    pub struct Ctype;
}

mod b {
    pub struct Atype<T, K>(T, K);
    pub struct Btype<T, K>(T, K);
    pub struct Ctype<T, K>(T, K);
}

use b::*;

fn main() {
    let x: Atype<
      Btype<
        Ctype<
          Atype<
            Btype<
              Ctype<
                Atype<
                  Btype<
                    Ctype<i32, i32>,
                    i32
                  >,
                  i32
                >,
                i32
              >,
              i32
            >,
            i32
          >,
          i32
        >,
        i32
      >,
      i32
    > = Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(
        Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(
            Ok("")
        ))))))))))))))))))))))))))))))
    )))))))))))))))))))))))))))))];
    //~^^^^^ ERROR E0308

    let _ = Some(Ok(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(
        Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(
            Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(
                Some(Some(Some(Some(Some(Some(Some(Some(Some("")))))))))
            )))))))))))))))))
        ))))))))))))))))))
    ))))))))))))))))) == Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(
        Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(
            Ok(Ok(Ok(Ok(Ok(Ok(Ok("")))))))
        ))))))))))))))))))))))))))))))
    )))))))))))))))))))))))];
    //~^^^^^ ERROR E0308

    let x: Atype<
      Btype<
        Ctype<
          Atype<
            Btype<
              Ctype<
                Atype<
                  Btype<
                    Ctype<i32, i32>,
                    i32
                  >,
                  i32
                >,
                i32
              >,
              i32
            >,
            i32
          >,
          i32
        >,
        i32
      >,
      i32
    > = ();
    //~^ ERROR E0308

    let _: () = Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(
        Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(
            Ok(Ok(Ok(Ok(Ok(Ok(Ok("")))))))
        ))))))))))))))))))))))))))))))
    )))))))))))))))))))))))];
    //~^^^^^ ERROR E0308
}
"#;

    let input_new = &[Group::with_title(Level::ERROR
        .title("mismatched types")
        .id("E0308")).element(
            Snippet::source(source)
                .line_start(7)
                .path("$DIR/long-E0308.rs")

                .annotation(
                    AnnotationKind::Primary
                        .span(719..1001)
                        .label("expected `Atype<Btype<Ctype<..., i32>, i32>, i32>`, found `Result<Result<Result<..., _>, _>, _>`"),
                )
                .annotation(
                    AnnotationKind::Context
                        .span(293..716)
                        .label("expected due to this"),
                )
        ).element(
            Level::NOTE
                .title("expected struct `Atype<Btype<..., i32>, i32>`\n     found enum `Result<Result<..., _>, _>`")
        ).element(
            Level::NOTE
                .title("the full name for the type has been written to '$TEST_BUILD_DIR/$FILE.long-type-hash.txt'")
        ).element(
            Level::NOTE
                .title("consider using `--verbose` to print the full type name to the console")
                ,
        )];

    let expected = str![[r#"
error[E0308]: mismatched types
   ╭▸ $DIR/long-E0308.rs:48:9
   │
LL │        let x: Atype<
   │ ┌─────────────┘
LL │ │        Btype<
LL │ │          Ctype<
LL │ │            Atype<
   ‡ │
LL │ │        i32
LL │ │      > = Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(O…
   │ │┏━━━━━│━━━┛
   │ └┃─────┤
   │  ┃     expected due to this
LL │  ┃         Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(O…
LL │  ┃             Ok("")
LL │  ┃         ))))))))))))))))))))))))))))))
LL │  ┃     )))))))))))))))))))))))))))))];
   │  ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ expected `Atype<Btype<Ctype<..., i32>, i32>, i32>`, found `Result<Result<Result<..., _>, _>, _>`
   │
   ├ note: expected struct `Atype<Btype<..., i32>, i32>`
   │            found enum `Result<Result<..., _>, _>`
   ├ note: the full name for the type has been written to '$TEST_BUILD_DIR/$FILE.long-type-hash.txt'
   ╰ note: consider using `--verbose` to print the full type name to the console
"#]];
    let renderer = Renderer::plain()
        .term_width(60)
        .theme(OutputTheme::Unicode)
        .anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input_new), expected);
}

#[test]
fn highlighting() {
    let source = r#"
use core::pin::Pin;
use core::future::Future;
use core::any::Any;

fn query(_: fn(Box<(dyn Any + Send + '_)>) -> Pin<Box<(
    dyn Future<Output = Result<Box<(dyn Any + 'static)>, String>> + Send + 'static
)>>) {}

fn wrapped_fn<'a>(_: Box<(dyn Any + Send)>) -> Pin<Box<(
    dyn Future<Output = Result<Box<(dyn Any + 'static)>, String>> + Send + 'static
)>> {
    Box::pin(async { Err("nope".into()) })
}

fn main() {
    query(wrapped_fn);
}
"#;

    let input_new = &[Group::with_title(Level::ERROR
        .title("mismatched types")
        .id("E0308")).element(
            Snippet::source(source)
                .line_start(7)
                .path("$DIR/unicode-output.rs")

                .annotation(
                    AnnotationKind::Primary
                        .span(430..440)
                        .label("one type is more general than the other"),
                )
                .annotation(
                    AnnotationKind::Context
                        .span(424..429)
                        .label("arguments to this function are incorrect"),
                ),
        ).element(
            Level::NOTE
                .title("expected fn pointer `for<'a> fn(Box<(dyn Any + Send + 'a)>) -> Pin<_>`\n      found fn item `fn(Box<(dyn Any + Send + 'static)>) -> Pin<_> {wrapped_fn}`")
                ,
        ),Group::with_title(
            Level::NOTE.title("function defined here"),
        ).element(
            Snippet::source(source)
                .line_start(7)
                .path("$DIR/unicode-output.rs")

                .annotation(AnnotationKind::Primary.span(77..210))
                .annotation(AnnotationKind::Context.span(71..76)),
        )];

    let expected = str![[r#"
error[E0308]: mismatched types
   ╭▸ $DIR/unicode-output.rs:23:11
   │
LL │     query(wrapped_fn);
   │     ┬──── ━━━━━━━━━━ one type is more general than the other
   │     │
   │     arguments to this function are incorrect
   │
   ╰ note: expected fn pointer `for<'a> fn(Box<(dyn Any + Send + 'a)>) -> Pin<_>`
                 found fn item `fn(Box<(dyn Any + Send + 'static)>) -> Pin<_> {wrapped_fn}`
note: function defined here
   ╭▸ $DIR/unicode-output.rs:12:10
   │
LL │   fn query(_: fn(Box<(dyn Any + Send + '_)>) -> Pin<Box<(
   │ ┏━━━━─────━┛
LL │ ┃     dyn Future<Output = Result<Box<(dyn Any + 'static)>, String>> + Send + 'static
LL │ ┃ )>>) {}
   ╰╴┗━━━┛
"#]];
    let renderer = Renderer::plain()
        .theme(OutputTheme::Unicode)
        .anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input_new), expected);
}

// This tests that an ellipsis is not inserted into Unicode text when a line
// wasn't actually trimmed.
//
// This is a regression test where `...` was inserted because the code wasn't
// properly accounting for the *rendered* length versus the length in bytes in
// all cases.
#[test]
fn unicode_cut_handling() {
    let source = "version = \"0.1.0\"\n# Ensure that the spans from toml handle utf-8 correctly\nauthors = [\n    { name = \"Z\u{351}\u{36b}\u{343}\u{36a}\u{302}\u{36b}\u{33d}\u{34f}\u{334}\u{319}\u{324}\u{31e}\u{349}\u{35a}\u{32f}\u{31e}\u{320}\u{34d}A\u{36b}\u{357}\u{334}\u{362}\u{335}\u{31c}\u{330}\u{354}L\u{368}\u{367}\u{369}\u{358}\u{320}G\u{311}\u{357}\u{30e}\u{305}\u{35b}\u{341}\u{334}\u{33b}\u{348}\u{34d}\u{354}\u{339}O\u{342}\u{30c}\u{30c}\u{358}\u{328}\u{335}\u{339}\u{33b}\u{31d}\u{333}\", email = 1 }\n]\n";
    let input = &[Group::with_title(Level::ERROR.title("title")).element(
        Snippet::source(source)
            .fold(false)
            .annotation(AnnotationKind::Primary.span(85..228).label("annotation")),
    )];
    let expected_ascii = str![[r#"
error: title
  |
1 |   version = "0.1.0"
2 |   # Ensure that the spans from toml handle utf-8 correctly
3 |   authors = [
  |  ___________^
4 | |     { name = "Z͑ͫ̓ͪ̂ͫ̽͏̴̙̤̞͉͚̯̞̠͍A̴̵̜̰͔ͫ͗͢L̠ͨͧͩ͘G̴̻͈͍͔̹̑͗̎̅͛́Ǫ̵̹̻̝̳͂̌̌͘", email = 1 }
5 | | ]
  | |_^ annotation
"#]];
    let renderer_ascii = Renderer::plain();
    assert_data_eq!(renderer_ascii.render(input), expected_ascii);

    let expected_unicode = str![[r#"
error: title
  ╭▸ 
1 │   version = "0.1.0"
2 │   # Ensure that the spans from toml handle utf-8 correctly
3 │   authors = [
  │ ┏━━━━━━━━━━━┛
4 │ ┃     { name = "Z͑ͫ̓ͪ̂ͫ̽͏̴̙̤̞͉͚̯̞̠͍A̴̵̜̰͔ͫ͗͢L̠ͨͧͩ͘G̴̻͈͍͔̹̑͗̎̅͛́Ǫ̵̹̻̝̳͂̌̌͘", email = 1 }
5 │ ┃ ]
  ╰╴┗━┛ annotation
"#]];
    let renderer_unicode = renderer_ascii.theme(OutputTheme::Unicode);
    assert_data_eq!(renderer_unicode.render(input), expected_unicode);
}

#[test]
fn unicode_cut_handling2() {
    let source = "/*这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。*/?";
    let input = &[Group::with_title(Level::ERROR
        .title("expected item, found `?`")).element(
                Snippet::source(source)
                    .fold(false)
                    .annotation(AnnotationKind::Primary.span(499..500).label("expected item"))
            ).element(
                Level::NOTE.title("for a full list of items that can appear in modules, see <https://doc.rust-lang.org/reference/items.html>")

       )];

    let expected_ascii = str![[r#"
error: expected item, found `?`
  |
1 | ... 的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。*/?
  |                                                              ^ expected item
  |
  = note: for a full list of items that can appear in modules, see <https://doc.rust-lang.org/reference/items.html>
"#]];

    let renderer_ascii = Renderer::plain();
    assert_data_eq!(renderer_ascii.render(input), expected_ascii);

    let expected_unicode = str![[r#"
error: expected item, found `?`
  ╭▸ 
1 │ … 宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。*/?
  │                                                              ━ expected item
  │
  ╰ note: for a full list of items that can appear in modules, see <https://doc.rust-lang.org/reference/items.html>
"#]];
    let renderer_unicode = renderer_ascii.theme(OutputTheme::Unicode);
    assert_data_eq!(renderer_unicode.render(input), expected_unicode);
}

#[test]
fn unicode_cut_handling3() {
    let source = "/*这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。这是宽的。*/?";
    let input = &[Group::with_title(Level::ERROR
        .title("expected item, found `?`")).element(
                Snippet::source(source)
                    .fold(false)
                    .annotation(AnnotationKind::Primary.span(251..254).label("expected item"))
            ).element(
                Level::NOTE.title("for a full list of items that can appear in modules, see <https://doc.rust-lang.org/reference/items.html>")

       )];

    let expected_ascii = str![[r#"
error: expected item, found `?`
  |
1 | ... 。这是宽的。这是宽的。这是宽的...
  |             ^^ expected item
  |
  = note: for a full list of items that can appear in modules, see <https://doc.rust-lang.org/reference/items.html>
"#]];

    let renderer_ascii = Renderer::plain().term_width(43);
    assert_data_eq!(renderer_ascii.render(input), expected_ascii);

    let expected_unicode = str![[r#"
error: expected item, found `?`
  ╭▸ 
1 │ … 的。这是宽的。这是宽的。这是宽的。…
  │             ━━ expected item
  │
  ╰ note: for a full list of items that can appear in modules, see <https://doc.rust-lang.org/reference/items.html>
"#]];
    let renderer_unicode = renderer_ascii.theme(OutputTheme::Unicode);
    assert_data_eq!(renderer_unicode.render(input), expected_unicode);
}

#[test]
fn unicode_cut_handling4() {
    let source = "/*aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa*/?";
    let input = &[Group::with_title(Level::ERROR
        .title("expected item, found `?`")).element(
                Snippet::source(source)
                    .fold(false)
                    .annotation(AnnotationKind::Primary.span(334..335).label("expected item"))
            ).element(
                Level::NOTE.title("for a full list of items that can appear in modules, see <https://doc.rust-lang.org/reference/items.html>")

       )];

    let expected_ascii = str![[r#"
error: expected item, found `?`
  |
1 | ...aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa*/?
  |                                                             ^ expected item
  |
  = note: for a full list of items that can appear in modules, see <https://doc.rust-lang.org/reference/items.html>
"#]];

    let renderer_ascii = Renderer::plain();
    assert_data_eq!(renderer_ascii.render(input), expected_ascii);

    let expected_unicode = str![[r#"
error: expected item, found `?`
  ╭▸ 
1 │ …aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa*/?
  │                                                             ━ expected item
  │
  ╰ note: for a full list of items that can appear in modules, see <https://doc.rust-lang.org/reference/items.html>
"#]];
    let renderer_unicode = renderer_ascii.theme(OutputTheme::Unicode);
    assert_data_eq!(renderer_unicode.render(input), expected_unicode);
}

#[test]
fn diagnostic_width() {
    let source = r##"// ignore-tidy-linelength

fn main() {
    let _: &str = "🦀☀☁☂☃☄★☆☇☈☉☊☋☌☍☎☏☐☑☒☓  ☖☗☘☙☚☛☜☝☞☟☠☡☢☣☤☥☦☧☨☩☪☫☬☭☮☯☰☱☲☳☴☵☶☷☸☹☺☻☼☽☾☿♀♁♂♃♄♅♆♇♏♔♕♖♗♘♙♚♛♜♝♞♟♠♡♢♣♤♥♦♧♨♩♪♫♬♭♮♯♰♱♲♳♴♵♶♷♸♹♺♻♼♽♾♿⚀⚁⚂⚃⚄⚅⚆⚈⚉4🦀☀☁☂☃☄★☆☇☈☉☊☋☌☍☎☏☐☑☒☓☖☗☘☙☚☛☜☝☞☟☠☡☢☣☤☥☦☧☨☩☪☫☬☭☮☯☰☱☲☳☴☵☶☷☸☹☺☻☼☽☾☿♀♁♂♃♄♅♆♇♏♔♕♖♗♘♙♚♛♜♝♞♟♠♡♢♣♤♥♦♧♨♩♪♫♬♭♮♯♰♱♲♳♴♵♶♷♸♹♺♻♼♽♾♿⚀⚁⚂⚃⚄⚅⚆⚈⚉4🦀🦀☁☂☃☄★☆☇☈☉☊☋☌☍☎☏☐☑☒☓☖☗☘☙☚☛☜☝☞☟☠☡☢☣☤☥☦☧☨☩☪☫☬☭☮☯☰☱☲☳☴☵☶☷☸☹☺☻☼☽☾☿♀♁♂♃♄♅♆♇♏♔♕♖♗♘♙♚♛♜♝♞♟♠♡♢♣♤♥♦♧♨♩♪♫♬♭♮♯♰♱♲♳♴♵♶♷♸♹♺♻♼♽♾♿⚀⚁⚂⚃⚄⚅⚆⚈⚉4"; let _: () = 42;  let _: &str = "🦀☀☁☂☃☄★☆☇☈☉☊☋☌☍☎☏☐☑☒☓  ☖☗☘☙☚☛☜☝☞☟☠☡☢☣☤☥☦☧☨☩☪☫☬☭☮☯☰☱☲☳☴☵☶☷☸☹☺☻☼☽☾☿♀♁♂♃♄♅♆♇♏♔♕♖♗♘♙♚♛♜♝♞♟♠♡♢♣♤♥♦♧♨♩♪♫♬♭♮♯♰♱♲♳♴♵♶♷♸♹♺♻♼♽♾♿⚀⚁⚂⚃⚄⚅⚆⚈⚉4🦀☀☁☂☃☄★☆☇☈☉☊☋☌☍☎☏☐☑☒☓☖☗☘☙☚☛☜☝☞☟☠☡☢☣☤☥☦☧☨☩☪☫☬☭☮☯☰☱☲☳☴☵☶☷☸☹☺☻☼☽☾☿♀♁♂♃♄♅♆♇♏♔♕♖♗♘♙♚♛♜♝♞♟♠♡♢♣♤♥♦♧♨♩♪♫♬♭♮♯♰♱♲♳♴♵♶♷♸♹♺♻♼♽♾♿⚀⚁⚂⚃⚄⚅⚆⚈⚉4🦀🦀☁☂☃☄★☆☇☈☉☊☋☌☍☎☏☐☑☒☓☖☗☘☙☚☛☜☝☞☟☠☡☢☣☤☥☦☧☨☩☪☫☬☭☮☯☰☱☲☳☴☵☶☷☸☹☺☻☼☽☾☿♀♁♂♃♄♅♆♇♏♔♕♖♗♘♙♚♛♜♝♞♟♠♡♢♣♤♥♦♧♨♩♪♫♬♭♮♯♰♱♲♳♴♵♶♷♸♹♺♻♼♽♾♿⚀⚁⚂⚃⚄⚅⚆⚈⚉4";
//~^ ERROR mismatched types
}
"##;
    let input = &[
        Group::with_title(Level::ERROR.title("mismatched types").id("E0308")).element(
            Snippet::source(source)
                .path("$DIR/non-whitespace-trimming-unicode.rs")
                .annotation(
                    AnnotationKind::Primary
                        .span(1207..1209)
                        .label("expected `()`, found integer"),
                )
                .annotation(
                    AnnotationKind::Context
                        .span(1202..1204)
                        .label("expected due to this"),
                ),
        ),
    ];

    let expected_ascii = str![[r#"
error[E0308]: mismatched types
  --> $DIR/non-whitespace-trimming-unicode.rs:4:415
   |
LL | ...♧♨♩♪♫♬♭♮♯♰♱♲♳♴♵♶♷♸♹♺♻♼♽♾♿⚀⚁⚂⚃⚄⚅⚆⚈⚉4"; let _: () = 42;  let _: &str = "🦀☀☁☂☃☄★☆☇☈☉☊☋☌☍☎☏☐☑☒☓  ☖☗☘☙☚☛☜☝☞☟☠☡☢☣☤☥☦☧☨☩☪☫☬☭☮☯☰☱☲☳☴☵☶☷...
   |                                                  --   ^^ expected `()`, found integer
   |                                                  |
   |                                                  expected due to this
"#]];

    let renderer_ascii = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer_ascii.render(input), expected_ascii);

    let expected_unicode = str![[r#"
error[E0308]: mismatched types
   ╭▸ $DIR/non-whitespace-trimming-unicode.rs:4:415
   │
LL │ …♥♦♧♨♩♪♫♬♭♮♯♰♱♲♳♴♵♶♷♸♹♺♻♼♽♾♿⚀⚁⚂⚃⚄⚅⚆⚈⚉4"; let _: () = 42;  let _: &str = "🦀☀☁☂☃☄★☆☇☈☉☊☋☌☍☎☏☐☑☒☓  ☖☗☘☙☚☛☜☝☞☟☠☡☢☣☤☥☦☧☨☩☪☫☬☭☮☯☰☱☲☳☴☵☶☷☸☹…
   │                                                  ┬─   ━━ expected `()`, found integer
   │                                                  │
   ╰╴                                                 expected due to this
"#]];
    let renderer_unicode = renderer_ascii.theme(OutputTheme::Unicode);
    assert_data_eq!(renderer_unicode.render(input), expected_unicode);
}

#[test]
fn diagnostic_width2() {
    let source = r##"//@ revisions: ascii unicode
//@[unicode] compile-flags: -Zunstable-options --error-format=human-unicode
// ignore-tidy-linelength

fn main() {
    let unicode_is_fun = "؁‱ஹ௸௵꧄.ဪ꧅⸻𒈙𒐫﷽𒌄𒈟𒍼𒁎𒀱𒌧𒅃 𒈓𒍙𒊎𒄡𒅌𒁏𒀰𒐪𒐩𒈙𒐫𪚥";
    let _ = "ༀ༁༂༃༄༅༆༇༈༉༊་༌།༎༏༐༑༒༓༔༕༖༗༘༙༚༛༜༝༞༟༠༡༢༣༤༥༦༧༨༩༪༫༬༭༮༯༰༱༲༳༴༵༶༷༸༹༺༻༼༽༾༿ཀཁགགྷངཅཆཇ཈ཉཊཋཌཌྷཎཏཐདདྷནཔཕབབྷམཙཚཛཛྷཝཞཟའཡརལཤཥསཧཨཀྵཪཫཬ཭཮཯཰ཱཱཱིིུུྲྀཷླྀཹེཻོཽཾཿ྄ཱྀྀྂྃ྅྆྇ྈྉྊྋྌྍྎྏྐྑྒྒྷྔྕྖྗ྘ྙྚྛྜྜྷྞྟྠྡྡྷྣྤྥྦྦྷྨྩྪྫྫྷྭྮྯྰྱྲླྴྵྶྷྸྐྵྺྻྼ྽྾྿࿀࿁࿂࿃࿄࿅࿆࿇࿈࿉࿊࿋࿌࿍࿎࿏࿐࿑࿒࿓࿔࿕࿖࿗࿘࿙࿚"; let _a = unicode_is_fun + " really fun!";
    //[ascii]~^ ERROR cannot add `&str` to `&str`
}
"##;
    let input = &[
        Group::with_title(
            Level::ERROR
                .title("cannot add `&str` to `&str`")
                .id("E0369"),
        )
        .element(
            Snippet::source(source)
                .path("$DIR/non-1-width-unicode-multiline-label.rs")
                .annotation(AnnotationKind::Context.span(970..984).label("&str"))
                .annotation(AnnotationKind::Context.span(987..1001).label("&str"))
                .annotation(
                    AnnotationKind::Primary
                        .span(985..986)
                        .label("`+` cannot be used to concatenate two `&str` strings"),
                ),
        )
        .element(
            Level::NOTE.message("string concatenation requires an owned `String` on the left"),
        ),
        Group::with_title(Level::HELP.title("create an owned `String` from a string reference"))
            .element(
                Snippet::source(source)
                    .path("$DIR/non-1-width-unicode-multiline-label.rs")
                    .patch(Patch::new(984..984, ".to_owned()")),
            ),
    ];

    let expected_ascii = str![[r#"
error[E0369]: cannot add `&str` to `&str`
  --> $DIR/non-1-width-unicode-multiline-label.rs:7:260
   |
LL | ...࿉࿊࿋࿌࿍࿎࿏࿐࿑࿒࿓࿔࿕࿖࿗࿘࿙࿚"; let _a = unicode_is_fun + " really fun!";
   |                                  -------------- ^ -------------- &str
   |                                  |              |
   |                                  |              `+` cannot be used to concatenate two `&str` strings
   |                                  &str
   |
   = note: string concatenation requires an owned `String` on the left
help: create an owned `String` from a string reference
   |
LL |     let _ = "ༀ༁༂༃༄༅༆༇༈༉༊་༌།༎༏༐༑༒༓༔༕༖༗༘༙༚༛༜༝༞༟༠༡༢༣༤༥༦༧༨༩༪༫༬༭༮༯༰༱༲༳༴༵༶༷༸༹༺༻༼༽༾༿ཀཁགགྷངཅཆཇ཈ཉཊཋཌཌྷཎཏཐདདྷནཔཕབབྷམཙཚཛཛྷཝཞཟའཡརལཤཥསཧཨཀྵཪཫཬ཭཮཯཰ཱཱཱིིུུྲྀཷླྀཹེཻོཽཾཿ྄ཱྀྀྂྃ྅྆྇ྈྉྊྋྌྍྎྏྐྑྒྒྷྔྕྖྗ྘ྙྚྛྜྜྷྞྟྠྡྡྷྣྤྥྦྦྷྨྩྪྫྫྷྭྮྯྰྱྲླྴྵྶྷྸྐྵྺྻྼ྽྾྿࿀࿁࿂࿃࿄࿅࿆࿇࿈࿉࿊࿋࿌࿍࿎࿏࿐࿑࿒࿓࿔࿕࿖࿗࿘࿙࿚"; let _a = unicode_is_fun.to_owned() + " really fun!";
   |                                                                                                                                                                                         +++++++++++
"#]];

    let renderer_ascii = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer_ascii.render(input), expected_ascii);

    let expected_unicode = str![[r#"
error[E0369]: cannot add `&str` to `&str`
   ╭▸ $DIR/non-1-width-unicode-multiline-label.rs:7:260
   │
LL │ …࿆࿇࿈࿉࿊࿋࿌࿍࿎࿏࿐࿑࿒࿓࿔࿕࿖࿗࿘࿙࿚"; let _a = unicode_is_fun + " really fun!";
   │                                  ┬───────────── ┯ ────────────── &str
   │                                  │              │
   │                                  │              `+` cannot be used to concatenate two `&str` strings
   │                                  &str
   │
   ╰ note: string concatenation requires an owned `String` on the left
help: create an owned `String` from a string reference
   ╭╴
LL │     let _ = "ༀ༁༂༃༄༅༆༇༈༉༊་༌།༎༏༐༑༒༓༔༕༖༗༘༙༚༛༜༝༞༟༠༡༢༣༤༥༦༧༨༩༪༫༬༭༮༯༰༱༲༳༴༵༶༷༸༹༺༻༼༽༾༿ཀཁགགྷངཅཆཇ཈ཉཊཋཌཌྷཎཏཐདདྷནཔཕབབྷམཙཚཛཛྷཝཞཟའཡརལཤཥསཧཨཀྵཪཫཬ཭཮཯཰ཱཱཱིིུུྲྀཷླྀཹེཻོཽཾཿ྄ཱྀྀྂྃ྅྆྇ྈྉྊྋྌྍྎྏྐྑྒྒྷྔྕྖྗ྘ྙྚྛྜྜྷྞྟྠྡྡྷྣྤྥྦྦྷྨྩྪྫྫྷྭྮྯྰྱྲླྴྵྶྷྸྐྵྺྻྼ྽྾྿࿀࿁࿂࿃࿄࿅࿆࿇࿈࿉࿊࿋࿌࿍࿎࿏࿐࿑࿒࿓࿔࿕࿖࿗࿘࿙࿚"; let _a = unicode_is_fun.to_owned() + " really fun!";
   ╰╴                                                                                                                                                                                        +++++++++++
"#]];

    let renderer_unicode = renderer_ascii.theme(OutputTheme::Unicode);
    assert_data_eq!(renderer_unicode.render(input), expected_unicode);
}

#[test]
fn macros_not_utf8() {
    let source = r##"//@ error-pattern: did not contain valid UTF-8
//@ reference: input.encoding.utf8
//@ reference: input.encoding.invalid

fn foo() {
    include!("not-utf8.bin");
}
"##;
    let bin_source = "�|�\u{0002}!5�cc\u{0015}\u{0002}�Ӻi��WWj�ȥ�'�}�\u{0012}�J�ȉ��W�\u{001e}O�@����\u{001c}w�V���LO����\u{0014}[ \u{0003}_�'���SQ�~ذ��ų&��-\t��lN~��!@␌ _#���kQ��h�\u{001d}�:�\u{001c}\u{0007}�";
    let input = &[Group::with_title(Level::ERROR
        .title("couldn't read `$DIR/not-utf8.bin`: stream did not contain valid UTF-8")).element(
                Snippet::source(source)
                    .path("$DIR/not-utf8.rs")

                    .annotation(AnnotationKind::Primary.span(136..160)),
            ),
            Group::with_title(Level::NOTE.title("byte `193` is not valid utf-8"))
                .element(
                    Snippet::source(bin_source)
                        .path("$DIR/not-utf8.bin")

                        .annotation(AnnotationKind::Primary.span(0..0)),
                )
                .element(Level::NOTE.message("this error originates in the macro `include` (in Nightly builds, run with -Z macro-backtrace for more info)")),
       ];

    let expected_ascii = str![[r#"
error: couldn't read `$DIR/not-utf8.bin`: stream did not contain valid UTF-8
  --> $DIR/not-utf8.rs:6:5
   |
LL |     include!("not-utf8.bin");
   |     ^^^^^^^^^^^^^^^^^^^^^^^^
   |
note: byte `193` is not valid utf-8
  --> $DIR/not-utf8.bin:1:1
   |
LL | �|�␂!5�cc␕␂�Ӻi��WWj�ȥ�'�}�␒�J�ȉ��W�␞O�@����␜w�V���LO����␔[ ␃_�'���SQ�~ذ��ų&��-    ��lN~��!@␌ _#���kQ��h�␝�:�␜␇�
   | ^
   = note: this error originates in the macro `include` (in Nightly builds, run with -Z macro-backtrace for more info)
"#]];

    let renderer_ascii = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer_ascii.render(input), expected_ascii);

    let expected_unicode = str![[r#"
error: couldn't read `$DIR/not-utf8.bin`: stream did not contain valid UTF-8
   ╭▸ $DIR/not-utf8.rs:6:5
   │
LL │     include!("not-utf8.bin");
   │     ━━━━━━━━━━━━━━━━━━━━━━━━
   ╰╴
note: byte `193` is not valid utf-8
   ╭▸ $DIR/not-utf8.bin:1:1
   │
LL │ �|�␂!5�cc␕␂�Ӻi��WWj�ȥ�'�}�␒�J�ȉ��W�␞O�@����␜w�V���LO����␔[ ␃_�'���SQ�~ذ��ų&��-    ��lN~��!@␌ _#���kQ��h�␝�:�␜␇�
   │ ━
   ╰ note: this error originates in the macro `include` (in Nightly builds, run with -Z macro-backtrace for more info)
"#]];
    let renderer_unicode = renderer_ascii.theme(OutputTheme::Unicode);
    assert_data_eq!(renderer_unicode.render(input), expected_unicode);
}

#[test]
fn secondary_title_no_level_text() {
    let source = r#"fn main() {
    let b: &[u8] = include_str!("file.txt");    //~ ERROR mismatched types
    let s: &str = include_bytes!("file.txt");   //~ ERROR mismatched types
}"#;

    let input = &[
        Group::with_title(Level::ERROR.title("mismatched types").id("E0308"))
            .element(
                Snippet::source(source)
                    .path("$DIR/mismatched-types.rs")
                    .annotation(
                        AnnotationKind::Primary
                            .span(105..131)
                            .label("expected `&str`, found `&[u8; 0]`"),
                    )
                    .annotation(
                        AnnotationKind::Context
                            .span(98..102)
                            .label("expected due to this"),
                    ),
            )
            .element(
                Level::NOTE
                    .no_name()
                    .message("expected reference `&str`\nfound reference `&'static [u8; 0]`"),
            ),
    ];

    let expected = str![[r#"
error[E0308]: mismatched types
  --> $DIR/mismatched-types.rs:3:19
   |
LL |     let s: &str = include_bytes!("file.txt");   //~ ERROR mismatched types
   |            ----   ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&str`, found `&[u8; 0]`
   |            |
   |            expected due to this
   |
   = expected reference `&str`
     found reference `&'static [u8; 0]`
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn secondary_title_custom_level_text() {
    let source = r#"fn main() {
    let b: &[u8] = include_str!("file.txt");    //~ ERROR mismatched types
    let s: &str = include_bytes!("file.txt");   //~ ERROR mismatched types
}"#;

    let input = &[
        Group::with_title(Level::ERROR.title("mismatched types").id("E0308"))
            .element(
                Snippet::source(source)
                    .path("$DIR/mismatched-types.rs")
                    .annotation(
                        AnnotationKind::Primary
                            .span(105..131)
                            .label("expected `&str`, found `&[u8; 0]`"),
                    )
                    .annotation(
                        AnnotationKind::Context
                            .span(98..102)
                            .label("expected due to this"),
                    ),
            )
            .element(
                Level::NOTE
                    .with_name(Some("custom"))
                    .message("expected reference `&str`\nfound reference `&'static [u8; 0]`"),
            ),
    ];

    let expected = str![[r#"
error[E0308]: mismatched types
  --> $DIR/mismatched-types.rs:3:19
   |
LL |     let s: &str = include_bytes!("file.txt");   //~ ERROR mismatched types
   |            ----   ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&str`, found `&[u8; 0]`
   |            |
   |            expected due to this
   |
   = custom: expected reference `&str`
             found reference `&'static [u8; 0]`
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn id_on_title() {
    let source = r#"// Regression test for issue #114529
// Tests that we do not ICE during const eval for a
// break-with-value in contexts where it is illegal

#[allow(while_true)]
fn main() {
    [(); {
        while true {
            break 9; //~ ERROR `break` with value from a `while` loop
        };
        51
    }];

    [(); {
        while let Some(v) = Some(9) {
            break v; //~ ERROR `break` with value from a `while` loop
        };
        51
    }];

    while true {
        break (|| { //~ ERROR `break` with value from a `while` loop
            let local = 9;
        });
    }
}
"#;
    let input = &[
        Group::with_title(
            Level::ERROR
                .title("`break` with value from a `while` loop")
                .id("E0571"),
        )
        .element(
            Snippet::source(source)
                .line_start(1)
                .path("$DIR/issue-114529-illegal-break-with-value.rs")
                .annotation(
                    AnnotationKind::Primary
                        .span(483..581)
                        .label("can only break with a value inside `loop` or breakable block"),
                )
                .annotation(
                    AnnotationKind::Context
                        .span(462..472)
                        .label("you can't `break` with a value in a `while` loop"),
                ),
        ),
        Group::with_title(
            Level::HELP
                .with_name(Some("suggestion"))
                .title("use `break` on its own without a value inside this `while` loop")
                .id("S0123"),
        )
        .element(
            Snippet::source(source)
                .line_start(1)
                .path("$DIR/issue-114529-illegal-break-with-value.rs")
                .patch(Patch::new(483..581, "break")),
        ),
    ];

    let expected_ascii = str![[r#"
error[E0571]: `break` with value from a `while` loop
  --> $DIR/issue-114529-illegal-break-with-value.rs:22:9
   |
LL |       while true {
   |       ---------- you can't `break` with a value in a `while` loop
LL | /         break (|| { //~ ERROR `break` with value from a `while` loop
LL | |             let local = 9;
LL | |         });
   | |__________^ can only break with a value inside `loop` or breakable block
   |
suggestion[S0123]: use `break` on its own without a value inside this `while` loop
   |
LL -         break (|| { //~ ERROR `break` with value from a `while` loop
LL -             let local = 9;
LL -         });
LL +         break;
   |
"#]];

    let renderer_ascii = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer_ascii.render(input), expected_ascii);

    let expected_unicode = str![[r#"
error[E0571]: `break` with value from a `while` loop
   ╭▸ $DIR/issue-114529-illegal-break-with-value.rs:22:9
   │
LL │       while true {
   │       ────────── you can't `break` with a value in a `while` loop
LL │ ┏         break (|| { //~ ERROR `break` with value from a `while` loop
LL │ ┃             let local = 9;
LL │ ┃         });
   │ ┗━━━━━━━━━━┛ can only break with a value inside `loop` or breakable block
   ╰╴
suggestion[S0123]: use `break` on its own without a value inside this `while` loop
   ╭╴
LL -         break (|| { //~ ERROR `break` with value from a `while` loop
LL -             let local = 9;
LL -         });
LL +         break;
   ╰╴
"#]];
    let renderer_unicode = renderer_ascii.theme(OutputTheme::Unicode);
    assert_data_eq!(renderer_unicode.render(input), expected_unicode);
}

#[test]
fn max_line_num_no_fold() {
    let source = r#"cargo
fuzzy
pizza
jumps
crazy
quack
zappy
"#;

    let input_new = &[Group::with_title(
        Level::ERROR
            .title("the size for values of type `T` cannot be known at compilation time")
            .id("E0277"),
    )
    .element(
        Snippet::source(source)
            .line_start(8)
            .fold(false)
            .annotation(AnnotationKind::Primary.span(6..11)),
    )];
    let expected = str![[r#"
error[E0277]: the size for values of type `T` cannot be known at compilation time
   |
 8 | cargo
 9 | fuzzy
   | ^^^^^
10 | pizza
11 | jumps
12 | crazy
13 | quack
14 | zappy
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input_new), expected);
}

#[test]
fn empty_span_start_line() {
    let source = "#: E112\nif False:\nprint()\n#: E113\nprint()\n";
    let input = &[Group::with_level(Level::ERROR).element(
        Snippet::source(source)
            .line_start(7)
            .fold(false)
            .annotation(AnnotationKind::Primary.span(18..18).label("E112")),
    )];

    let expected = str![[r#"
   |
 7 | #: E112
 8 | if False:
 9 | print()
   | ^ E112
10 | #: E113
11 | print()
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn suggestion_span_one_bigger_than_source() {
    let snippet_source = r#"#![allow(unused)]
fn main() {
[1, 2, 3].into_iter().for_each(|n| { *n; });
}
"#;

    let suggestion_source = r#"[1, 2, 3].into_iter().for_each(|n| { *n; });
"#;

    let long_title1 ="this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to `<[T; N] as IntoIterator>::into_iter` in Rust 2021";
    let long_title2 = "for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>";
    let long_title3 = "or use `IntoIterator::into_iter(..)` instead of `.into_iter()` to explicitly iterate by value";

    let input = &[
        Group::with_title(Level::WARNING.title(long_title1))
            .element(
                Snippet::source(snippet_source)
                    .path("lint_example.rs")
                    .annotation(AnnotationKind::Primary.span(40..49)),
            )
            .element(Level::WARNING.message("this changes meaning in Rust 2021"))
            .element(Level::NOTE.message(long_title2))
            .element(Level::NOTE.message("`#[warn(array_into_iter)]` on by default")),
        Group::with_title(
            Level::HELP.title("use `.iter()` instead of `.into_iter()` to avoid ambiguity"),
        )
        .element(
            Snippet::source(suggestion_source)
                .path("lint_example.rs")
                .line_start(3)
                .patch(Patch::new(10..19, "iter")),
        ),
        Group::with_title(Level::HELP.title(long_title3)).element(
            Snippet::source(suggestion_source)
                .path("lint_example.rs")
                .line_start(3)
                .patch(Patch::new(
                    suggestion_source.len() + 1..suggestion_source.len() + 1,
                    "IntoIterator::into_iter(",
                )),
        ),
    ];

    let expected = str![[r#"
warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to `<[T; N] as IntoIterator>::into_iter` in Rust 2021
 --> lint_example.rs:3:11
  |
3 | [1, 2, 3].into_iter().for_each(|n| { *n; });
  |           ^^^^^^^^^
  |
  = warning: this changes meaning in Rust 2021
  = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
  = note: `#[warn(array_into_iter)]` on by default
help: use `.iter()` instead of `.into_iter()` to avoid ambiguity
  |
3 - [1, 2, 3].into_iter().for_each(|n| { *n; });
3 + [1, 2, 3].iter().for_each(|n| { *n; });
  |
help: or use `IntoIterator::into_iter(..)` instead of `.into_iter()` to explicitly iterate by value
  |
3 | IntoIterator::into_iter(
  |
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
#[should_panic = "Patch span `47..47` is beyond the end of buffer `45`"]
fn suggestion_span_bigger_than_source() {
    let snippet_source = r#"#![allow(unused)]
fn main() {
[1, 2, 3].into_iter().for_each(|n| { *n; });
}
"#;
    let suggestion_source = r#"[1, 2, 3].into_iter().for_each(|n| { *n; });
"#;

    let long_title1 ="this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to `<[T; N] as IntoIterator>::into_iter` in Rust 2021";
    let long_title2 = "for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>";
    let long_title3 = "or use `IntoIterator::into_iter(..)` instead of `.into_iter()` to explicitly iterate by value";

    let input = &[
        Group::with_title(Level::WARNING.title(long_title1))
            .element(
                Snippet::source(snippet_source)
                    .path("lint_example.rs")
                    .annotation(AnnotationKind::Primary.span(40..49)),
            )
            .element(Level::WARNING.message("this changes meaning in Rust 2021"))
            .element(Level::NOTE.message(long_title2))
            .element(Level::NOTE.message("`#[warn(array_into_iter)]` on by default")),
        Group::with_title(
            Level::HELP.title("use `.iter()` instead of `.into_iter()` to avoid ambiguity"),
        )
        .element(
            Snippet::source(suggestion_source)
                .path("lint_example.rs")
                .line_start(3)
                .patch(Patch::new(10..19, "iter")),
        ),
        Group::with_title(Level::HELP.title(long_title3)).element(
            Snippet::source(suggestion_source)
                .path("lint_example.rs")
                .line_start(3)
                .patch(Patch::new(
                    suggestion_source.len() + 2..suggestion_source.len() + 2,
                    "IntoIterator::into_iter(",
                )),
        ),
    ];

    let renderer = Renderer::plain();
    renderer.render(input);
}

#[test]
fn snippet_no_path() {
    // Taken from: https://docs.python.org/3/library/typing.html#annotating-callable-objects

    let source = "def __call__(self, *vals: bytes, maxlen: int | None = None) -> list[bytes]: ...";
    let input = &[Group::with_title(Level::ERROR.title("")).element(
        Snippet::source(source).annotation(AnnotationKind::Primary.span(4..12).label("annotation")),
    )];

    let expected_ascii = str![[r#"
error: 
  |
1 | def __call__(self, *vals: bytes, maxlen: int | None = None) -> list[bytes]: ...
  |     ^^^^^^^^ annotation
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected_ascii);

    let expected_unicode = str![[r#"
error: 
  ╭▸ 
1 │ def __call__(self, *vals: bytes, maxlen: int | None = None) -> list[bytes]: ...
  ╰╴    ━━━━━━━━ annotation
"#]];
    let renderer = Renderer::plain().theme(OutputTheme::Unicode);
    assert_data_eq!(renderer.render(input), expected_unicode);
}

#[test]
fn multiple_snippet_no_path() {
    // Taken from: https://docs.python.org/3/library/typing.html#annotating-callable-objects

    let source = "def __call__(self, *vals: bytes, maxlen: int | None = None) -> list[bytes]: ...";
    let input = &[Group::with_title(Level::ERROR.title(""))
        .element(
            Snippet::source(source)
                .annotation(AnnotationKind::Primary.span(4..12).label("annotation")),
        )
        .element(
            Snippet::source(source)
                .annotation(AnnotationKind::Primary.span(4..12).label("annotation")),
        )];

    let expected_ascii = str![[r#"
error: 
  |
1 | def __call__(self, *vals: bytes, maxlen: int | None = None) -> list[bytes]: ...
  |     ^^^^^^^^ annotation
  |
 ::: 
1 | def __call__(self, *vals: bytes, maxlen: int | None = None) -> list[bytes]: ...
  |     ^^^^^^^^ annotation
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected_ascii);

    let expected_unicode = str![[r#"
error: 
  ╭▸ 
1 │ def __call__(self, *vals: bytes, maxlen: int | None = None) -> list[bytes]: ...
  │     ━━━━━━━━ annotation
  │
  ⸬  
1 │ def __call__(self, *vals: bytes, maxlen: int | None = None) -> list[bytes]: ...
  ╰╴    ━━━━━━━━ annotation
"#]];
    let renderer = Renderer::plain().theme(OutputTheme::Unicode);
    assert_data_eq!(renderer.render(input), expected_unicode);
}

#[test]
fn padding_last_in_group() {
    let source = r#"// When the type of a method call's receiver is unknown, the span should point
// to the receiver (and not the entire call, as was previously the case before
// the fix of which this tests).

fn shines_a_beacon_through_the_darkness() {
    let x: Option<_> = None; //~ ERROR type annotations needed
    x.unwrap().method_that_could_exist_on_some_type();
}

fn courier_to_des_moines_and_points_west(data: &[u32]) -> String {
    data.iter()
        .sum::<_>() //~ ERROR type annotations needed
        .to_string()
}

fn main() {}
"#;

    let input = &[
        Group::with_title(Level::ERROR.title("type annotations needed").id("E0282"))
            .element(
                Snippet::source(source)
                    .path("$DIR/issue-42234-unknown-receiver-type.rs")
                    .annotation(AnnotationKind::Primary.span(449..452).label(
                        "cannot infer type of the type parameter `S` declared on the method `sum`",
                    )),
            )
            .element(Padding),
    ];

    let expected_ascii = str![[r#"
error[E0282]: type annotations needed
  --> $DIR/issue-42234-unknown-receiver-type.rs:12:10
   |
LL |         .sum::<_>() //~ ERROR type annotations needed
   |          ^^^ cannot infer type of the type parameter `S` declared on the method `sum`
   |
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input), expected_ascii);

    let expected_unicode = str![[r#"
error[E0282]: type annotations needed
   ╭▸ $DIR/issue-42234-unknown-receiver-type.rs:12:10
   │
LL │         .sum::<_>() //~ ERROR type annotations needed
   │          ━━━ cannot infer type of the type parameter `S` declared on the method `sum`
   ╰╴
"#]];
    let renderer = renderer.theme(OutputTheme::Unicode);
    assert_data_eq!(renderer.render(input), expected_unicode);
}

#[test]
fn padding_last_in_group_with_group_after() {
    let source = r#"// When the type of a method call's receiver is unknown, the span should point
// to the receiver (and not the entire call, as was previously the case before
// the fix of which this tests).

fn shines_a_beacon_through_the_darkness() {
    let x: Option<_> = None; //~ ERROR type annotations needed
    x.unwrap().method_that_could_exist_on_some_type();
}

fn courier_to_des_moines_and_points_west(data: &[u32]) -> String {
    data.iter()
        .sum::<_>() //~ ERROR type annotations needed
        .to_string()
}

fn main() {}
"#;

    let input = &[
        Group::with_title(Level::ERROR.title("type annotations needed").id("E0282"))
            .element(
                Snippet::source(source)
                    .path("$DIR/issue-42234-unknown-receiver-type.rs")
                    .annotation(AnnotationKind::Primary.span(449..452).label(
                        "cannot infer type of the type parameter `S` declared on the method `sum`",
                    )),
            )
            .element(Padding),
        Group::with_title(Level::HELP.title("consider specifying the generic argument")).element(
            Snippet::source(source)
                .path("$DIR/issue-42234-unknown-receiver-type.rs")
                .line_start(12)
                .fold(true)
                .patch(Patch::new(452..457, "::<GENERIC_ARG>")),
        ),
    ];

    let expected_ascii = str![[r#"
error[E0282]: type annotations needed
  --> $DIR/issue-42234-unknown-receiver-type.rs:12:10
   |
LL |         .sum::<_>() //~ ERROR type annotations needed
   |          ^^^ cannot infer type of the type parameter `S` declared on the method `sum`
   |
help: consider specifying the generic argument
   |
LL -         .sum::<_>() //~ ERROR type annotations needed
LL +         .sum::<GENERIC_ARG>() //~ ERROR type annotations needed
   |
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input), expected_ascii);

    let expected_unicode = str![[r#"
error[E0282]: type annotations needed
   ╭▸ $DIR/issue-42234-unknown-receiver-type.rs:12:10
   │
LL │         .sum::<_>() //~ ERROR type annotations needed
   │          ━━━ cannot infer type of the type parameter `S` declared on the method `sum`
   ╰╴
help: consider specifying the generic argument
   ╭╴
LL -         .sum::<_>() //~ ERROR type annotations needed
LL +         .sum::<GENERIC_ARG>() //~ ERROR type annotations needed
   ╰╴
"#]];
    let renderer = renderer.theme(OutputTheme::Unicode);
    assert_data_eq!(renderer.render(input), expected_unicode);
}

#[test]
fn suggestion_same_as_source() {
    let source = r#"// When the type of a method call's receiver is unknown, the span should point
// to the receiver (and not the entire call, as was previously the case before
// the fix of which this tests).

fn shines_a_beacon_through_the_darkness() {
    let x: Option<_> = None; //~ ERROR type annotations needed
    x.unwrap().method_that_could_exist_on_some_type();
}

fn courier_to_des_moines_and_points_west(data: &[u32]) -> String {
    data.iter()
        .sum::<_>() //~ ERROR type annotations needed
        .to_string()
}

fn main() {}
"#;

    let input = &[
        Group::with_title(Level::ERROR.title("type annotations needed").id("E0282")).element(
            Snippet::source(source)
                .path("$DIR/issue-42234-unknown-receiver-type.rs")
                .annotation(AnnotationKind::Primary.span(449..452).label(
                    "cannot infer type of the type parameter `S` declared on the method `sum`",
                )),
        ),
        Group::with_title(Level::HELP.title("consider specifying the generic argument")).element(
            Snippet::source(source)
                .path("$DIR/issue-42234-unknown-receiver-type.rs")
                .line_start(12)
                .fold(true)
                .patch(Patch::new(452..457, "::<_>")),
        ),
    ];
    let expected = str![[r#"
error[E0282]: type annotations needed
  --> $DIR/issue-42234-unknown-receiver-type.rs:12:10
   |
LL |         .sum::<_>() //~ ERROR type annotations needed
   |          ^^^ cannot infer type of the type parameter `S` declared on the method `sum`
   |
help: consider specifying the generic argument
   |
LL |         .sum::<_>() //~ ERROR type annotations needed
   |
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn keep_lines1() {
    let source = r#"
cargo
fuzzy
pizza
jumps
crazy
quack
zappy
"#;

    let input_new = &[Group::with_title(
        Level::ERROR
            .title("the size for values of type `T` cannot be known at compilation time")
            .id("E0277"),
    )
    .element(
        Snippet::source(source)
            .line_start(11)
            .annotation(AnnotationKind::Primary.span(1..6))
            .annotation(AnnotationKind::Visible.span(37..41)),
    )];
    let expected = str![[r#"
error[E0277]: the size for values of type `T` cannot be known at compilation time
   |
12 | cargo
   | ^^^^^
...
18 | zappy
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input_new), expected);
}

#[test]
fn keep_lines2() {
    let source = r#"
cargo
fuzzy
pizza
jumps
crazy
quack
zappy
"#;

    let input_new = &[Group::with_title(
        Level::ERROR
            .title("the size for values of type `T` cannot be known at compilation time")
            .id("E0277"),
    )
    .element(
        Snippet::source(source)
            .line_start(11)
            .annotation(AnnotationKind::Primary.span(1..6))
            .annotation(AnnotationKind::Visible.span(16..18)),
    )];
    let expected = str![[r#"
error[E0277]: the size for values of type `T` cannot be known at compilation time
   |
12 | cargo
   | ^^^^^
13 | fuzzy
14 | pizza
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input_new), expected);
}
