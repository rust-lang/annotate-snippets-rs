use annotate_snippets::{Annotation, AnnotationKind, Group, Level, Renderer, Snippet};

use snapbox::{assert_data_eq, str};

#[test]
fn test_i_29() {
    let snippets = Level::Error.message("oops").group(
        Group::new().element(
            Snippet::source("First line\r\nSecond oops line")
                .origin("<current file>")
                .annotation(AnnotationKind::Primary.span(19..23).label("oops"))
                .fold(true),
        ),
    );
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
    let snippets = Level::Error.message("").group(
        Group::new().element(
            Snippet::source("こんにちは、世界")
                .origin("<current file>")
                .annotation(AnnotationKind::Primary.span(18..24).label("world")),
        ),
    );

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
    let snippets = Level::Error.message("").group(
        Group::new().element(
            Snippet::source("おはよう\nございます")
                .origin("<current file>")
                .annotation(AnnotationKind::Primary.span(6..22).label("Good morning")),
        ),
    );

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
    let snippets = Level::Error.message("").group(
        Group::new().element(
            Snippet::source("お寿司\n食べたい🍣")
                .origin("<current file>")
                .annotation(AnnotationKind::Primary.span(0..9).label("Sushi1"))
                .annotation(AnnotationKind::Context.span(16..22).label("Sushi2")),
        ),
    );

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
    let snippets = Level::Error.message("").group(
        Group::new().element(
            Snippet::source("こんにちは、新しいWorld！")
                .origin("<current file>")
                .annotation(AnnotationKind::Primary.span(18..32).label("New world")),
        ),
    );

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
    let input = Level::Error.message("This is a title").id("E0001");

    let expected = str![r#"error[E0001]: This is a title"#];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn test_format_snippet_only() {
    let source = "This is line 1\nThis is line 2";
    let input = Level::Error
        .message("")
        .group(Group::new().element(Snippet::<Annotation<'_>>::source(source).line_start(5402)));

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
    let input = Level::Error.message("").group(
        Group::new()
            .element(
                Snippet::<Annotation<'_>>::source(src_0)
                    .line_start(5402)
                    .origin("file1.rs"),
            )
            .element(
                Snippet::<Annotation<'_>>::source(src_1)
                    .line_start(2)
                    .origin("file2.rs"),
            ),
    );
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
    let input = Level::Error.message("").group(
        Group::new().element(
            Snippet::source(&source).line_start(5402).annotation(
                AnnotationKind::Context
                    .span(range.clone())
                    .label("Test annotation"),
            ),
        ),
    );
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
    let input = Level::Error
        .message("")
        .group(Group::new().element(Level::Error.title("This __is__ a title")));
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
    let input = Level::Error.message("").group(
        Group::new().element(
            Snippet::source(source).line_start(0).annotation(
                AnnotationKind::Primary
                    .span(0..source.len() + 2)
                    .label(label),
            ),
        ),
    );
    let renderer = Renderer::plain();
    let _ = renderer.render(input);
}

#[test]
fn test_source_content() {
    let source = "This is an example\nof content lines";
    let input = Level::Error
        .message("")
        .group(Group::new().element(Snippet::<Annotation<'_>>::source(source).line_start(56)));
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
    let input = Level::Error.message("").group(
        Group::new().element(
            Snippet::source(source)
                .line_start(1)
                .annotation(AnnotationKind::Context.span(0..5).label("Example string")),
        ),
    );
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
    let input = Level::Error.message("").group(
        Group::new().element(
            Snippet::source(source)
                .line_start(1)
                .annotation(AnnotationKind::Context.span(0..5).label("Example string"))
                .annotation(AnnotationKind::Context.span(0..5).label("Second line")),
        ),
    );
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
    let input = Level::Error
        .message("")
        .group(Group::new().element(Snippet::<Annotation<'_>>::source("").origin("file.rs")));
    let expected = str![[r#"
error: 
 --> file.rs
  |
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn test_anon_lines() {
    let source = "This is an example\nof content lines\n\nabc";
    let input = Level::Error
        .message("")
        .group(Group::new().element(Snippet::<Annotation<'_>>::source(source).line_start(56)));
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
    let input = Level::Error.message("dummy").group(
        Group::new().element(
            Snippet::source("foo\nbar\nbaz")
                .origin("file/path")
                .line_start(3)
                .fold(true)
                .annotation(AnnotationKind::Primary.span(4..11)),
        ), // bar\nbaz
    );

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
    let input = Level::Error.message("").group(
        Group::new().element(
            Snippet::source(source)
                .origin("file/path")
                .line_start(3)
                .fold(true)
                .annotation(AnnotationKind::Primary.span(0..10)),
        ), // 1..10 works
    );
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
    let input = Level::Error.message("").group(
        Group::new().element(
            Snippet::source(source)
                .origin("file/path")
                .line_start(3)
                .annotation(AnnotationKind::Primary.span(0..2)),
        ), // a\r
    );
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
    let input = Level::Error.message("").group(
        Group::new().element(
            Snippet::source(source)
                .origin("file/path")
                .line_start(3)
                .annotation(AnnotationKind::Primary.span(0..3)),
        ), // a\r\n
    );
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
fn char_eol_annotate_char_double_width() {
    let snippets = Level::Error.message("").group(
        Group::new().element(
            Snippet::source("こん\r\nにちは\r\n世界")
                .origin("<current file>")
                .annotation(AnnotationKind::Primary.span(3..8)),
        ), // ん\r\n
    );

    let expected = str![[r#"
error: 
 --> <current file>:1:2
  |
1 | こん
  |   ^^
2 | にちは
3 | 世界
"#]];

    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(snippets), expected);
}

#[test]
fn annotate_eol() {
    let source = "a\r\nb";
    let input = Level::Error.message("").group(
        Group::new().element(
            Snippet::source(source)
                .origin("file/path")
                .line_start(3)
                .annotation(AnnotationKind::Primary.span(1..2)),
        ), // \r
    );
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
    let input = Level::Error.message("").group(
        Group::new().element(
            Snippet::source(source)
                .origin("file/path")
                .line_start(3)
                .annotation(AnnotationKind::Primary.span(1..3)),
        ), // \r\n
    );
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
fn annotate_eol3() {
    let source = "a\r\nb";
    let input = Level::Error.message("").group(
        Group::new().element(
            Snippet::source(source)
                .origin("file/path")
                .line_start(3)
                .annotation(AnnotationKind::Primary.span(2..3)),
        ), // \n
    );
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
fn annotate_eol4() {
    let source = "a\r\nb";
    let input = Level::Error.message("").group(
        Group::new().element(
            Snippet::source(source)
                .origin("file/path")
                .line_start(3)
                .annotation(AnnotationKind::Primary.span(2..2)),
        ), // \n
    );
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
    let snippets = Level::Error.message("").group(
        Group::new().element(
            Snippet::source("こん\r\nにちは\r\n世界")
                .origin("<current file>")
                .annotation(AnnotationKind::Primary.span(7..8)),
        ), // \n
    );

    let expected = str![[r#"
error: 
 --> <current file>:1:4
  |
1 | こん
  |     ^
2 | にちは
3 | 世界
"#]];

    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(snippets), expected);
}

#[test]
fn multiline_eol_start() {
    let source = "a\r\nb";
    let input = Level::Error.message("").group(
        Group::new().element(
            Snippet::source(source)
                .origin("file/path")
                .line_start(3)
                .annotation(AnnotationKind::Primary.span(1..4)),
        ), // \r\nb
    );
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
    let input = Level::Error.message("").group(
        Group::new().element(
            Snippet::source(source)
                .origin("file/path")
                .line_start(3)
                .annotation(AnnotationKind::Primary.span(2..4)),
        ), // \nb
    );
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
    let input = Level::Error.message("").group(
        Group::new().element(
            Snippet::source(source)
                .origin("file/path")
                .line_start(3)
                .annotation(AnnotationKind::Primary.span(1..3)),
        ), // \nb
    );
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
    let snippets = Level::Error.message("").group(
        Group::new().element(
            Snippet::source("こん\r\nにちは\r\n世界")
                .origin("<current file>")
                .annotation(AnnotationKind::Primary.span(7..11)),
        ), // \r\nに
    );

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
    let input = Level::Error.message("").group(
        Group::new().element(
            Snippet::source(source)
                .origin("file/path")
                .line_start(3)
                .annotation(AnnotationKind::Primary.span(1..4)),
        ), // \nb\n
    );
    let expected = str![[r#"
error: 
 --> file/path:3:2
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
fn multiline_eol_start_eol_end2() {
    let source = "a\r\nb\r\nc";
    let input = Level::Error.message("").group(
        Group::new().element(
            Snippet::source(source)
                .origin("file/path")
                .line_start(3)
                .annotation(AnnotationKind::Primary.span(2..5)),
        ), // \nb\r
    );
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
    let input = Level::Error.message("").group(
        Group::new().element(
            Snippet::source(source)
                .origin("file/path")
                .line_start(3)
                .annotation(AnnotationKind::Primary.span(2..6)),
        ), // \nb\r\n
    );
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
fn multiline_eol_start_eof_end() {
    let source = "a\r\nb";
    let input = Level::Error.message("").group(
        Group::new().element(
            Snippet::source(source)
                .origin("file/path")
                .line_start(3)
                .annotation(AnnotationKind::Primary.span(1..5)),
        ), // \r\nb(EOF)
    );
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
    let input = Level::Error.message("").group(
        Group::new().element(
            Snippet::source(source)
                .origin("file/path")
                .line_start(3)
                .annotation(AnnotationKind::Primary.span(3..9)),
        ), // \r\nに(EOF)
    );
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
    let input = Level::Error.message("unused optional dependency").group(
        Group::new().element(
            Snippet::source(source)
                .origin("Cargo.toml")
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
    );
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
    let input = Level::Error.message("unused optional dependency").group(
        Group::new().element(
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
    );
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
    let input = Level::Error.message("unused optional dependency").group(
        Group::new().element(
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
    );
    let expected = str![[r#"
error: unused optional dependency
  |
4 |    bar = { version = "0.1.0", optional = true }
  |   _________^__________________--------------^
  |  |         |                  |
  |  |_________|                  This should also be long but not too long
  | ||
5 | || this is another line
6 | || so is this
7 | || bar = { version = "0.1.0", optional = true }
  | ||_________________________^________________^ I need this to be really long so I can test overlaps
  | |__________________________|
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
    let input = Level::Error.message("unused optional dependency").group(
        Group::new().element(
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
    );
    let expected = str![[r#"
error: unused optional dependency
  |
4 |     bar = { version = "0.1.0", optional = true }
  |   __________^__________________--------------^
  |  |          |                  |
  |  |__________|                  This should also be long but not too long
  | ||
5 | ||  this is another line
  | || ____^
6 | ||| so is this
7 | ||| bar = { version = "0.1.0", optional = true }
  | |||_________________________^________________^ I need this to be really long so I can test overlaps
  | |_|_________________________|
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
    let input = Level::Error.message("title").group(
        Group::new().element(
            Snippet::source(source)
                .origin("origin.txt")
                .fold(false)
                .annotation(AnnotationKind::Primary.span(8..8 + 3).label("annotation")),
        ),
    );

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
    let input = Level::Error.message("title").group(
        Group::new().element(
            Snippet::source(source)
                .origin("origin.txt")
                .fold(false)
                .annotation(
                    AnnotationKind::Primary
                        .span(8 + 1..8 + 3)
                        .label("annotation"),
                ),
        ),
    );

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
