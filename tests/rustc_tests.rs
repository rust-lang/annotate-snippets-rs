//! These tests have been adapted from [Rust's parser tests][parser-tests].
//!
//! [parser-tests]: https://github.com/rust-lang/rust/blob/894f7a4ba6554d3797404bbf550d9919df060b97/compiler/rustc_parse/src/parser/tests.rs

use annotate_snippets::{level::Level, AnnotationKind, Group, Origin, Renderer, Snippet};

use snapbox::{assert_data_eq, str};

#[test]
fn ends_on_col0() {
    let source = r#"
fn foo() {
}
"#;
    let input = Level::ERROR.message("foo").group(
        Group::new().element(
            Snippet::source(source)
                .line_start(1)
                .origin("test.rs")
                .fold(true)
                .annotation(AnnotationKind::Primary.span(10..13).label("test")),
        ),
    );

    let expected = str![[r#"
error: foo
 --> test.rs:2:10
  |
2 |   fn foo() {
  |  __________^
3 | | }
  | |_^ test
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}
#[test]
fn ends_on_col2() {
    let source = r#"
fn foo() {


  }
"#;
    let input = Level::ERROR.message("foo").group(
        Group::new().element(
            Snippet::source(source)
                .line_start(1)
                .origin("test.rs")
                .fold(true)
                .annotation(AnnotationKind::Primary.span(10..17).label("test")),
        ),
    );

    let expected = str![[r#"
error: foo
 --> test.rs:2:10
  |
2 |   fn foo() {
  |  __________^
... |
5 | |   }
  | |___^ test
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}
#[test]
fn non_nested() {
    let source = r#"
fn foo() {
  X0 Y0
  X1 Y1
  X2 Y2
}
"#;
    let input = Level::ERROR.message("foo").group(
        Group::new().element(
            Snippet::source(source)
                .line_start(1)
                .origin("test.rs")
                .fold(true)
                .annotation(
                    AnnotationKind::Primary
                        .span(14..32)
                        .label("`X` is a good letter"),
                )
                .annotation(
                    AnnotationKind::Context
                        .span(17..35)
                        .label("`Y` is a good letter too"),
                ),
        ),
    );

    let expected = str![[r#"
error: foo
 --> test.rs:3:3
  |
3 |      X0 Y0
  |  ____^  -
  | | ______|
4 | ||   X1 Y1
5 | ||   X2 Y2
  | ||____^__- `Y` is a good letter too
  | |_____|
  |       `X` is a good letter
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}
#[test]
fn nested() {
    let source = r#"
fn foo() {
  X0 Y0
  Y1 X1
}
"#;
    let input = Level::ERROR.message("foo").group(
        Group::new().element(
            Snippet::source(source)
                .line_start(1)
                .origin("test.rs")
                .fold(true)
                .annotation(
                    AnnotationKind::Primary
                        .span(14..27)
                        .label("`X` is a good letter"),
                )
                .annotation(
                    AnnotationKind::Context
                        .span(17..24)
                        .label("`Y` is a good letter too"),
                ),
        ),
    );

    let expected = str![[r#"
error: foo
 --> test.rs:3:3
  |
3 |      X0 Y0
  |  ____^  -
  | | ______|
4 | ||   Y1 X1
  | ||____-__^ `X` is a good letter
  |  |____|
  |       `Y` is a good letter too
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}
#[test]
fn different_overlap() {
    let source = r#"
fn foo() {
  X0 Y0 Z0
  X1 Y1 Z1
  X2 Y2 Z2
  X3 Y3 Z3
}
"#;
    let input = Level::ERROR.message("foo").group(
        Group::new().element(
            Snippet::source(source)
                .line_start(1)
                .origin("test.rs")
                .fold(true)
                .annotation(
                    AnnotationKind::Primary
                        .span(17..38)
                        .label("`X` is a good letter"),
                )
                .annotation(
                    AnnotationKind::Context
                        .span(31..49)
                        .label("`Y` is a good letter too"),
                ),
        ),
    );

    let expected = str![[r#"
error: foo
 --> test.rs:3:6
  |
3 |      X0 Y0 Z0
  |  _______^
4 | |    X1 Y1 Z1
  | | _________-
5 | ||   X2 Y2 Z2
  | ||____^ `X` is a good letter
6 |  |   X3 Y3 Z3
  |  |____- `Y` is a good letter too
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}
#[test]
fn triple_overlap() {
    let source = r#"
fn foo() {
  X0 Y0 Z0
  X1 Y1 Z1
  X2 Y2 Z2
}
"#;
    let input = Level::ERROR.message("foo").group(
        Group::new().element(
            Snippet::source(source)
                .line_start(1)
                .origin("test.rs")
                .fold(true)
                .annotation(
                    AnnotationKind::Primary
                        .span(14..38)
                        .label("`X` is a good letter"),
                )
                .annotation(
                    AnnotationKind::Context
                        .span(17..41)
                        .label("`Y` is a good letter too"),
                )
                .annotation(AnnotationKind::Context.span(20..44).label("`Z` label")),
        ),
    );

    let expected = str![[r#"
error: foo
 --> test.rs:3:3
  |
3 |       X0 Y0 Z0
  |  _____^  -  -
  | | _______|  |
  | || _________|
4 | |||   X1 Y1 Z1
5 | |||   X2 Y2 Z2
  | |||____^__-__- `Z` label
  | ||_____|__|
  | |______|  `Y` is a good letter too
  |        `X` is a good letter
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}
#[test]
fn triple_exact_overlap() {
    let source = r#"
fn foo() {
  X0 Y0 Z0
  X1 Y1 Z1
  X2 Y2 Z2
}
"#;
    let input = Level::ERROR.message("foo").group(
        Group::new().element(
            Snippet::source(source)
                .line_start(1)
                .origin("test.rs")
                .fold(true)
                .annotation(
                    AnnotationKind::Primary
                        .span(14..38)
                        .label("`X` is a good letter"),
                )
                .annotation(
                    AnnotationKind::Context
                        .span(14..38)
                        .label("`Y` is a good letter too"),
                )
                .annotation(AnnotationKind::Context.span(14..38).label("`Z` label")),
        ),
    );

    // This should have a `^` but we currently don't support the idea of a
    // "primary" annotation, which would solve this
    let expected = str![[r#"
error: foo
 --> test.rs:3:3
  |
3 | /   X0 Y0 Z0
4 | |   X1 Y1 Z1
5 | |   X2 Y2 Z2
  | |    ^
  | |    |
  | |    `X` is a good letter
  | |____`Y` is a good letter too
  |      `Z` label
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}
#[test]
fn minimum_depth() {
    let source = r#"
fn foo() {
  X0 Y0 Z0
  X1 Y1 Z1
  X2 Y2 Z2
  X3 Y3 Z3
}
"#;
    let input = Level::ERROR.message("foo").group(
        Group::new().element(
            Snippet::source(source)
                .line_start(1)
                .origin("test.rs")
                .fold(true)
                .annotation(
                    AnnotationKind::Primary
                        .span(17..27)
                        .label("`X` is a good letter"),
                )
                .annotation(
                    AnnotationKind::Context
                        .span(28..44)
                        .label("`Y` is a good letter too"),
                )
                .annotation(AnnotationKind::Context.span(36..52).label("`Z`")),
        ),
    );

    let expected = str![[r#"
error: foo
 --> test.rs:3:6
  |
3 |      X0 Y0 Z0
  |  _______^
4 | |    X1 Y1 Z1
  | | ____^_-
  | ||____|
  |  |    `X` is a good letter
5 |  |   X2 Y2 Z2
  |  |___-______- `Y` is a good letter too
  |   ___|
  |  |
6 |  |   X3 Y3 Z3
  |  |_______- `Z`
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}
#[test]
fn non_overlapping() {
    let source = r#"
fn foo() {
  X0 Y0 Z0
  X1 Y1 Z1
  X2 Y2 Z2
  X3 Y3 Z3
}
"#;
    let input = Level::ERROR.message("foo").group(
        Group::new().element(
            Snippet::source(source)
                .line_start(1)
                .origin("test.rs")
                .fold(true)
                .annotation(
                    AnnotationKind::Primary
                        .span(14..27)
                        .label("`X` is a good letter"),
                )
                .annotation(
                    AnnotationKind::Context
                        .span(39..55)
                        .label("`Y` is a good letter too"),
                ),
        ),
    );

    let expected = str![[r#"
error: foo
 --> test.rs:3:3
  |
3 | /   X0 Y0 Z0
4 | |   X1 Y1 Z1
  | |____^ `X` is a good letter
5 |     X2 Y2 Z2
  |  ______-
6 | |   X3 Y3 Z3
  | |__________- `Y` is a good letter too
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}
#[test]
fn overlapping_start_and_end() {
    let source = r#"
fn foo() {
  X0 Y0 Z0
  X1 Y1 Z1
  X2 Y2 Z2
  X3 Y3 Z3
}
"#;
    let input = Level::ERROR.message("foo").group(
        Group::new().element(
            Snippet::source(source)
                .line_start(1)
                .origin("test.rs")
                .fold(true)
                .annotation(
                    AnnotationKind::Primary
                        .span(17..27)
                        .label("`X` is a good letter"),
                )
                .annotation(
                    AnnotationKind::Context
                        .span(31..55)
                        .label("`Y` is a good letter too"),
                ),
        ),
    );

    let expected = str![[r#"
error: foo
 --> test.rs:3:6
  |
3 |      X0 Y0 Z0
  |  _______^
4 | |    X1 Y1 Z1
  | | ____^____-
  | ||____|
  |  |    `X` is a good letter
5 |  |   X2 Y2 Z2
6 |  |   X3 Y3 Z3
  |  |__________- `Y` is a good letter too
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}
#[test]
fn multiple_labels_primary_without_message() {
    let source = r#"
fn foo() {
  a { b { c } d }
}
"#;
    let input = Level::ERROR.message("foo").group(
        Group::new().element(
            Snippet::source(source)
                .line_start(1)
                .origin("test.rs")
                .fold(true)
                .annotation(AnnotationKind::Primary.span(18..25).label(""))
                .annotation(
                    AnnotationKind::Context
                        .span(14..27)
                        .label("`a` is a good letter"),
                )
                .annotation(AnnotationKind::Context.span(22..23).label("")),
        ),
    );

    let expected = str![[r#"
error: foo
 --> test.rs:3:7
  |
3 |   a { b { c } d }
  |   ----^^^^-^^-- `a` is a good letter
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}
#[test]
fn multiple_labels_secondary_without_message() {
    let source = r#"
fn foo() {
  a { b { c } d }
}
"#;
    let input = Level::ERROR.message("foo").group(
        Group::new().element(
            Snippet::source(source)
                .line_start(1)
                .origin("test.rs")
                .fold(true)
                .annotation(
                    AnnotationKind::Primary
                        .span(14..27)
                        .label("`a` is a good letter"),
                )
                .annotation(AnnotationKind::Context.span(18..25).label("")),
        ),
    );

    let expected = str![[r#"
error: foo
 --> test.rs:3:3
  |
3 |   a { b { c } d }
  |   ^^^^-------^^ `a` is a good letter
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}
#[test]
fn multiple_labels_primary_without_message_2() {
    let source = r#"
fn foo() {
  a { b { c } d }
}
"#;
    let input = Level::ERROR.message("foo").group(
        Group::new().element(
            Snippet::source(source)
                .line_start(1)
                .origin("test.rs")
                .fold(true)
                .annotation(
                    AnnotationKind::Primary
                        .span(18..25)
                        .label("`b` is a good letter"),
                )
                .annotation(AnnotationKind::Context.span(14..27).label(""))
                .annotation(AnnotationKind::Context.span(22..23).label("")),
        ),
    );

    let expected = str![[r#"
error: foo
 --> test.rs:3:7
  |
3 |   a { b { c } d }
  |   ----^^^^-^^--
  |       |
  |       `b` is a good letter
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}
#[test]
fn multiple_labels_secondary_without_message_2() {
    let source = r#"
fn foo() {
  a { b { c } d }
}
"#;
    let input = Level::ERROR.message("foo").group(
        Group::new().element(
            Snippet::source(source)
                .line_start(1)
                .origin("test.rs")
                .fold(true)
                .annotation(AnnotationKind::Primary.span(14..27).label(""))
                .annotation(
                    AnnotationKind::Context
                        .span(18..25)
                        .label("`b` is a good letter"),
                ),
        ),
    );

    let expected = str![[r#"
error: foo
 --> test.rs:3:3
  |
3 |   a { b { c } d }
  |   ^^^^-------^^
  |       |
  |       `b` is a good letter
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}
#[test]
fn multiple_labels_secondary_without_message_3() {
    let source = r#"
fn foo() {
  a  bc  d
}
"#;
    let input = Level::ERROR.message("foo").group(
        Group::new().element(
            Snippet::source(source)
                .line_start(1)
                .origin("test.rs")
                .fold(true)
                .annotation(
                    AnnotationKind::Primary
                        .span(14..18)
                        .label("`a` is a good letter"),
                )
                .annotation(AnnotationKind::Context.span(18..22).label("")),
        ),
    );

    let expected = str![[r#"
error: foo
 --> test.rs:3:3
  |
3 |   a  bc  d
  |   ^^^^----
  |   |
  |   `a` is a good letter
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}
#[test]
fn multiple_labels_without_message() {
    let source = r#"
fn foo() {
  a { b { c } d }
}
"#;
    let input = Level::ERROR.message("foo").group(
        Group::new().element(
            Snippet::source(source)
                .line_start(1)
                .origin("test.rs")
                .fold(true)
                .annotation(AnnotationKind::Primary.span(14..27).label(""))
                .annotation(AnnotationKind::Context.span(18..25).label("")),
        ),
    );

    let expected = str![[r#"
error: foo
 --> test.rs:3:3
  |
3 |   a { b { c } d }
  |   ^^^^-------^^
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}
#[test]
fn multiple_labels_without_message_2() {
    let source = r#"
fn foo() {
  a { b { c } d }
}
"#;
    let input = Level::ERROR.message("foo").group(
        Group::new().element(
            Snippet::source(source)
                .line_start(1)
                .origin("test.rs")
                .fold(true)
                .annotation(AnnotationKind::Primary.span(18..25).label(""))
                .annotation(AnnotationKind::Context.span(14..27).label(""))
                .annotation(AnnotationKind::Context.span(22..23).label("")),
        ),
    );

    let expected = str![[r#"
error: foo
 --> test.rs:3:7
  |
3 |   a { b { c } d }
  |   ----^^^^-^^--
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}
#[test]
fn multiple_labels_with_message() {
    let source = r#"
fn foo() {
  a { b { c } d }
}
"#;
    let input = Level::ERROR.message("foo").group(
        Group::new().element(
            Snippet::source(source)
                .line_start(1)
                .origin("test.rs")
                .fold(true)
                .annotation(
                    AnnotationKind::Primary
                        .span(14..27)
                        .label("`a` is a good letter"),
                )
                .annotation(
                    AnnotationKind::Context
                        .span(18..25)
                        .label("`b` is a good letter"),
                ),
        ),
    );

    let expected = str![[r#"
error: foo
 --> test.rs:3:3
  |
3 |   a { b { c } d }
  |   ^^^^-------^^
  |   |   |
  |   |   `b` is a good letter
  |   `a` is a good letter
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}
#[test]
fn ingle_label_with_message() {
    let source = r#"
fn foo() {
  a { b { c } d }
}
"#;
    let input = Level::ERROR.message("foo").group(
        Group::new().element(
            Snippet::source(source)
                .line_start(1)
                .origin("test.rs")
                .fold(true)
                .annotation(
                    AnnotationKind::Primary
                        .span(14..27)
                        .label("`a` is a good letter"),
                ),
        ),
    );

    let expected = str![[r#"
error: foo
 --> test.rs:3:3
  |
3 |   a { b { c } d }
  |   ^^^^^^^^^^^^^ `a` is a good letter
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}
#[test]
fn single_label_without_message() {
    let source = r#"
fn foo() {
  a { b { c } d }
}
"#;
    let input = Level::ERROR.message("foo").group(
        Group::new().element(
            Snippet::source(source)
                .line_start(1)
                .origin("test.rs")
                .fold(true)
                .annotation(AnnotationKind::Primary.span(14..27).label("")),
        ),
    );

    let expected = str![[r#"
error: foo
 --> test.rs:3:3
  |
3 |   a { b { c } d }
  |   ^^^^^^^^^^^^^
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}
#[test]
fn long_snippet() {
    let source = r#"
fn foo() {
  X0 Y0 Z0
  X1 Y1 Z1
1
2
3
4
5
6
7
8
9
10
  X2 Y2 Z2
  X3 Y3 Z3
}
"#;
    let input = Level::ERROR.message("foo").group(
        Group::new().element(
            Snippet::source(source)
                .line_start(1)
                .origin("test.rs")
                .fold(true)
                .annotation(
                    AnnotationKind::Primary
                        .span(17..27)
                        .label("`X` is a good letter"),
                )
                .annotation(
                    AnnotationKind::Context
                        .span(31..76)
                        .label("`Y` is a good letter too"),
                ),
        ),
    );

    let expected = str![[r#"
error: foo
  --> test.rs:3:6
   |
 3 |      X0 Y0 Z0
   |  _______^
 4 | |    X1 Y1 Z1
   | | ____^____-
   | ||____|
   |  |    `X` is a good letter
 5 |  | 1
 6 |  | 2
 7 |  | 3
...   |
15 |  |   X2 Y2 Z2
16 |  |   X3 Y3 Z3
   |  |__________- `Y` is a good letter too
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}
#[test]
fn long_snippet_multiple_spans() {
    let source = r#"
fn foo() {
  X0 Y0 Z0
1
2
3
  X1 Y1 Z1
4
5
6
  X2 Y2 Z2
7
8
9
10
  X3 Y3 Z3
}
"#;
    let input = Level::ERROR.message("foo").group(
        Group::new().element(
            Snippet::source(source)
                .line_start(1)
                .origin("test.rs")
                .fold(true)
                .annotation(
                    AnnotationKind::Primary
                        .span(17..73)
                        .label("`Y` is a good letter"),
                )
                .annotation(
                    AnnotationKind::Context
                        .span(37..56)
                        .label("`Z` is a good letter too"),
                ),
        ),
    );

    let expected = str![[r#"
error: foo
  --> test.rs:3:6
   |
 3 |      X0 Y0 Z0
   |  _______^
 4 | |  1
 5 | |  2
 6 | |  3
 7 | |    X1 Y1 Z1
   | | _________-
 8 | || 4
 9 | || 5
10 | || 6
11 | ||   X2 Y2 Z2
   | ||__________- `Z` is a good letter too
...  |
15 | |  10
16 | |    X3 Y3 Z3
   | |________^ `Y` is a good letter
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn issue_91334() {
    let source = r#"// Regression test for the ICE described in issue #91334.

//@ error-pattern: this file contains an unclosed delimiter

#![feature(coroutines)]

fn f(){||yield(((){),
"#;
    let input = Level::ERROR
        .message("this file contains an unclosed delimiter")
        .group(
            Group::new().element(
                Snippet::source(source)
                    .line_start(1)
                    .origin("$DIR/issue-91334.rs")
                    .fold(true)
                    .annotation(
                        AnnotationKind::Context
                            .span(151..152)
                            .label("unclosed delimiter"),
                    )
                    .annotation(
                        AnnotationKind::Context
                            .span(159..160)
                            .label("unclosed delimiter"),
                    )
                    .annotation(
                        AnnotationKind::Context
                            .span(164..164)
                            .label("missing open `(` for this delimiter"),
                    )
                    .annotation(AnnotationKind::Primary.span(167..167)),
            ),
        );
    let expected = str![[r#"
error: this file contains an unclosed delimiter
  --> $DIR/issue-91334.rs:7:23
   |
LL | fn f(){||yield(((){),
   |       -       -    - ^
   |       |       |    |
   |       |       |    missing open `(` for this delimiter
   |       |       unclosed delimiter
   |       unclosed delimiter
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn issue_114529_illegal_break_with_value() {
    // tests/ui/typeck/issue-114529-illegal-break-with-value.rs
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
    let input = Level::ERROR
        .message("`break` with value from a `while` loop")
        .id("E0571")
        .group(
            Group::new().element(
                Snippet::source(source)
                    .line_start(1)
                    .origin("$DIR/issue-114529-illegal-break-with-value.rs")
                    .fold(true)
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
        )
        .group(
            Group::new()
                .element(
                    Level::HELP
                        .title("use `break` on its own without a value inside this `while` loop"),
                )
                .element(
                    Snippet::source(source)
                        .line_start(1)
                        .origin("$DIR/issue-114529-illegal-break-with-value.rs")
                        .fold(true)
                        .annotation(AnnotationKind::Context.span(483..581).label("break")),
                ),
        );
    let expected = str![[r#"
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
help: use `break` on its own without a value inside this `while` loop
  --> $DIR/issue-114529-illegal-break-with-value.rs:22:9
   |
LL | /         break (|| { //~ ERROR `break` with value from a `while` loop
LL | |             let local = 9;
LL | |         });
   | |__________- break
"#]];

    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn primitive_reprs_should_have_correct_length() {
    // tests/ui/transmutability/enums/repr/primitive_reprs_should_have_correct_length.rs
    let source = r#"//! An enum with a primitive repr should have exactly the size of that primitive.

#![crate_type = "lib"]
#![feature(transmutability)]
#![allow(dead_code)]

mod assert {
    use std::mem::{Assume, TransmuteFrom};

    pub fn is_transmutable<Src, Dst>()
    where
        Dst: TransmuteFrom<Src, {
            Assume {
                alignment: true,
                lifetimes: true,
                safety: true,
                validity: true,
            }
        }>
    {}
}

#[repr(C)]
struct Zst;

#[derive(Clone, Copy)]
#[repr(i8)] enum V0i8 { V }
#[repr(u8)] enum V0u8 { V }
#[repr(i16)] enum V0i16 { V }
#[repr(u16)] enum V0u16 { V }
#[repr(i32)] enum V0i32 { V }
#[repr(u32)] enum V0u32 { V }
#[repr(i64)] enum V0i64 { V }
#[repr(u64)] enum V0u64 { V }
#[repr(isize)] enum V0isize { V }
#[repr(usize)] enum V0usize { V }

fn n8() {
    type Smaller = Zst;
    type Analog = u8;
    type Larger = u16;

    fn i_should_have_correct_length() {
        type Current = V0i8;

        assert::is_transmutable::<Smaller, Current>(); //~ ERROR cannot be safely transmuted
        assert::is_transmutable::<Current, Analog>();
        assert::is_transmutable::<Current, Larger>(); //~ ERROR cannot be safely transmuted
    }

    fn u_should_have_correct_length() {
        type Current = V0u8;

        assert::is_transmutable::<Smaller, Current>(); //~ ERROR cannot be safely transmuted
        assert::is_transmutable::<Current, Analog>();
        assert::is_transmutable::<Current, Larger>(); //~ ERROR cannot be safely transmuted
    }
}

fn n16() {
    type Smaller = u8;
    type Analog = u16;
    type Larger = u32;

    fn i_should_have_correct_length() {
        type Current = V0i16;

        assert::is_transmutable::<Smaller, Current>(); //~ ERROR cannot be safely transmuted
        assert::is_transmutable::<Current, Analog>();
        assert::is_transmutable::<Current, Larger>(); //~ ERROR cannot be safely transmuted
    }

    fn u_should_have_correct_length() {
        type Current = V0u16;

        assert::is_transmutable::<Smaller, Current>(); //~ ERROR cannot be safely transmuted
        assert::is_transmutable::<Current, Analog>();
        assert::is_transmutable::<Current, Larger>(); //~ ERROR cannot be safely transmuted
    }
}

fn n32() {
    type Smaller = u16;
    type Analog = u32;
    type Larger = u64;

    fn i_should_have_correct_length() {
        type Current = V0i32;

        assert::is_transmutable::<Smaller, Current>(); //~ ERROR cannot be safely transmuted
        assert::is_transmutable::<Current, Analog>();
        assert::is_transmutable::<Current, Larger>(); //~ ERROR cannot be safely transmuted
    }

    fn u_should_have_correct_length() {
        type Current = V0u32;

        assert::is_transmutable::<Smaller, Current>(); //~ ERROR cannot be safely transmuted
        assert::is_transmutable::<Current, Analog>();
        assert::is_transmutable::<Current, Larger>(); //~ ERROR cannot be safely transmuted
    }
}

fn n64() {
    type Smaller = u32;
    type Analog = u64;
    type Larger = u128;

    fn i_should_have_correct_length() {
        type Current = V0i64;

        assert::is_transmutable::<Smaller, Current>(); //~ ERROR cannot be safely transmuted
        assert::is_transmutable::<Current, Analog>();
        assert::is_transmutable::<Current, Larger>(); //~ ERROR cannot be safely transmuted
    }

    fn u_should_have_correct_length() {
        type Current = V0u64;

        assert::is_transmutable::<Smaller, Current>(); //~ ERROR cannot be safely transmuted
        assert::is_transmutable::<Current, Analog>();
        assert::is_transmutable::<Current, Larger>(); //~ ERROR cannot be safely transmuted
    }
}

fn nsize() {
    type Smaller = u8;
    type Analog = usize;
    type Larger = [usize; 2];

    fn i_should_have_correct_length() {
        type Current = V0isize;

        assert::is_transmutable::<Smaller, Current>(); //~ ERROR cannot be safely transmuted
        assert::is_transmutable::<Current, Analog>();
        assert::is_transmutable::<Current, Larger>(); //~ ERROR cannot be safely transmuted
    }

    fn u_should_have_correct_length() {
        type Current = V0usize;

        assert::is_transmutable::<Smaller, Current>(); //~ ERROR cannot be safely transmuted
        assert::is_transmutable::<Current, Analog>();
        assert::is_transmutable::<Current, Larger>(); //~ ERROR cannot be safely transmuted
    }
}
"#;
    let input =
        Level::ERROR
            .message("`V0usize` cannot be safely transmuted into `[usize; 2]`")
            .id("E0277")
            .group(
                Group::new().element(
                    Snippet::source(source)
                        .line_start(1)
                        .origin("$DIR/primitive_reprs_should_have_correct_length.rs")
                        .fold(true)
                        .annotation(AnnotationKind::Primary.span(4375..4381).label(
                            "the size of `V0usize` is smaller than the size of `[usize; 2]`",
                        )),
                ),
            )
            .group(
                Group::new()
                    .element(Level::NOTE.title("required by a bound in `is_transmutable`"))
                    .element(
                        Snippet::source(source)
                            .line_start(1)
                            .origin("$DIR/primitive_reprs_should_have_correct_length.rs")
                            .fold(true)
                            .annotation(
                                AnnotationKind::Context
                                    .span(225..240)
                                    .label("required by a bound in this function"),
                            )
                            .annotation(
                                AnnotationKind::Primary
                                    .span(276..470)
                                    .label("required by this bound in `is_transmutable`"),
                            ),
                    ),
            );
    let expected = str![[r#"
error[E0277]: `V0usize` cannot be safely transmuted into `[usize; 2]`
  --> $DIR/primitive_reprs_should_have_correct_length.rs:144:44
   |
LL |         assert::is_transmutable::<Current, Larger>(); //~ ERROR cannot be safely transmuted
   |                                            ^^^^^^ the size of `V0usize` is smaller than the size of `[usize; 2]`
   |
note: required by a bound in `is_transmutable`
  --> $DIR/primitive_reprs_should_have_correct_length.rs:12:14
   |
LL |       pub fn is_transmutable<Src, Dst>()
   |              --------------- required by a bound in this function
LL |       where
LL |           Dst: TransmuteFrom<Src, {
   |  ______________^
LL | |             Assume {
LL | |                 alignment: true,
LL | |                 lifetimes: true,
...  |
LL | |         }>
   | |__________^ required by this bound in `is_transmutable`
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn align_fail() {
    // tests/ui/transmutability/alignment/align-fail.rs
    let source = r#"//@ check-fail
#![feature(transmutability)]

mod assert {
    use std::mem::{Assume, TransmuteFrom};

    pub fn is_maybe_transmutable<Src, Dst>()
    where
        Dst: TransmuteFrom<Src, {
            Assume {
                alignment: false,
                lifetimes: true,
                safety: true,
                validity: true,
            }
        }>
    {}
}

fn main() {
    assert::is_maybe_transmutable::<&'static [u8; 0], &'static [u16; 0]>(); //~ ERROR `&[u8; 0]` cannot be safely transmuted into `&[u16; 0]`
}
"#;
    let input = Level::ERROR
        .message("`&[u8; 0]` cannot be safely transmuted into `&[u16; 0]`")
        .id("E027s7")
        .group(
            Group::new().element(
                Snippet::source(source)
                    .line_start(1)
                    .fold(true)
                    .origin("$DIR/align-fail.rs")
                    .annotation(
                        AnnotationKind::Primary
                            .span(442..459)
                            .label("the minimum alignment of `&[u8; 0]` (1) should be greater than that of `&[u16; 0]` (2)")
                    ),
            ),
        );
    let expected = str![[r#"
error[E027s7]: `&[u8; 0]` cannot be safely transmuted into `&[u16; 0]`
  --> $DIR/align-fail.rs:21:55
   |
LL | ...ic [u8; 0], &'static [u16; 0]>(); //~ ERROR `&[u8; 0]` cannot be safely transmuted into `&[u16; 0]`
   |                ^^^^^^^^^^^^^^^^^ the minimum alignment of `&[u8; 0]` (1) should be greater than that of `&[u16; 0]` (2)
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn missing_semicolon() {
    // tests/ui/suggestions/missing-semicolon.rs
    let source = r#"//@ run-rustfix
#![allow(dead_code, unused_variables, path_statements)]
fn a() {
    let x = 5;
    let y = x //~ ERROR expected function
    () //~ ERROR expected `;`, found `}`
}

fn b() {
    let x = 5;
    let y = x //~ ERROR expected function
    ();
}
fn c() {
    let x = 5;
    x //~ ERROR expected function
    ()
}
fn d() { // ok
    let x = || ();
    x
    ()
}
fn e() { // ok
    let x = || ();
    x
    ();
}
fn f()
 {
    let y = 5 //~ ERROR expected function
    () //~ ERROR expected `;`, found `}`
}
fn g() {
    5 //~ ERROR expected function
    ();
}
fn main() {}
"#;
    let input = Level::ERROR
        .message("expected function, found `{integer}`")
        .id("E0618")
        .group(
            Group::new().element(
                Snippet::source(source)
                    .line_start(1)
                    .origin("$DIR/missing-semicolon.rs")
                    .fold(true)
                    .annotation(
                        AnnotationKind::Context
                            .span(108..144)
                            .label("call expression requires function"),
                    )
                    .annotation(
                        AnnotationKind::Context
                            .span(89..90)
                            .label("`x` has type `{integer}`"),
                    )
                    .annotation(AnnotationKind::Context.span(109..109).label(
                        "help: consider using a semicolon here to finish the statement: `;`",
                    ))
                    .annotation(AnnotationKind::Primary.span(108..109)),
            ),
        );
    let expected = str![[r#"
error[E0618]: expected function, found `{integer}`
  --> $DIR/missing-semicolon.rs:5:13
   |
LL |       let x = 5;
   |           - `x` has type `{integer}`
LL |       let y = x //~ ERROR expected function
   |               ^- help: consider using a semicolon here to finish the statement: `;`
   |  _____________|
   | |
LL | |     () //~ ERROR expected `;`, found `}`
   | |______- call expression requires function
"#]];

    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn nested_macro_rules() {
    // tests/ui/proc-macro/nested-macro-rules.rs
    let source = r#"//@ run-pass
//@ aux-build:nested-macro-rules.rs
//@ proc-macro: test-macros.rs
//@ compile-flags: -Z span-debug -Z macro-backtrace
//@ edition:2018

#![no_std] // Don't load unnecessary hygiene information from std
#![warn(non_local_definitions)]

extern crate std;

extern crate nested_macro_rules;
extern crate test_macros;

use test_macros::{print_bang, print_attr};

use nested_macro_rules::FirstStruct;
struct SecondStruct;

fn main() {
    nested_macro_rules::inner_macro!(print_bang, print_attr);

    nested_macro_rules::outer_macro!(SecondStruct, SecondAttrStruct);
    //~^ WARN non-local `macro_rules!` definition
    inner_macro!(print_bang, print_attr);
}
"#;

    let aux_source = r#"pub struct FirstStruct;

#[macro_export]
macro_rules! outer_macro {
    ($name:ident, $attr_struct_name:ident) => {
        #[macro_export]
        macro_rules! inner_macro {
            ($bang_macro:ident, $attr_macro:ident) => {
                $bang_macro!($name);
                #[$attr_macro] struct $attr_struct_name {}
            }
        }
    }
}

outer_macro!(FirstStruct, FirstAttrStruct);
"#;
    let input = Level::WARNING
        .message("non-local `macro_rules!` definition, `#[macro_export]` macro should be written at top level module")
        .group(
            Group::new()
                .element(
                    Snippet::source(aux_source)
                        .line_start(1)
                        .origin("$DIR/auxiliary/nested-macro-rules.rs")
                        .fold(true)
                        .annotation(
                            AnnotationKind::Context
                                .span(41..65)
                                .label("in this expansion of `nested_macro_rules::outer_macro!`"),
                        )
                        .annotation(AnnotationKind::Primary.span(148..350)),
                )
                .element(
                    Snippet::source(source)
                        .line_start(1)
                        .origin("$DIR/nested-macro-rules.rs")
                        .fold(true)
                        .annotation(
                            AnnotationKind::Context
                                .span(510..574)
                                .label("in this macro invocation"),
                        ),
                )
                .element(
                    Level::HELP
                        .title("remove the `#[macro_export]` or move this `macro_rules!` outside the of the current function `main`")
                )
                .element(
                    Level::NOTE
                        .title("a `macro_rules!` definition is non-local if it is nested inside an item and has a `#[macro_export]` attribute")
                ),
        )
        .group(
            Group::new()
                .element(Level::NOTE.title("the lint level is defined here"))
                .element(
                    Snippet::source(source)
                        .line_start(1)
                        .origin("$DIR/nested-macro-rules.rs")
                        .fold(true)
                        .annotation(AnnotationKind::Primary.span(224..245)),
                ),
        );
    let expected = str![[r#"
warning: non-local `macro_rules!` definition, `#[macro_export]` macro should be written at top level module
  --> $DIR/auxiliary/nested-macro-rules.rs:7:9
   |
LL |   macro_rules! outer_macro {
   |   ------------------------ in this expansion of `nested_macro_rules::outer_macro!`
...
LL | /         macro_rules! inner_macro {
LL | |             ($bang_macro:ident, $attr_macro:ident) => {
LL | |                 $bang_macro!($name);
LL | |                 #[$attr_macro] struct $attr_struct_name {}
LL | |             }
LL | |         }
   | |_________^
   |
  ::: $DIR/nested-macro-rules.rs:23:5
   |
LL |       nested_macro_rules::outer_macro!(SecondStruct, SecondAttrStruct);
   |       ---------------------------------------------------------------- in this macro invocation
   |
   = help: remove the `#[macro_export]` or move this `macro_rules!` outside the of the current function `main`
   = note: a `macro_rules!` definition is non-local if it is nested inside an item and has a `#[macro_export]` attribute
note: the lint level is defined here
  --> $DIR/nested-macro-rules.rs:8:9
   |
LL | #![warn(non_local_definitions)]
   |         ^^^^^^^^^^^^^^^^^^^^^
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn method_on_ambiguous_numeric_type() {
    // tests/ui/methods/method-on-ambiguous-numeric-type.rs
    let source = r#"//@ aux-build:macro-in-other-crate.rs

#[macro_use] extern crate macro_in_other_crate;

macro_rules! local_mac {
    ($ident:ident) => { let $ident = 42; }
}
macro_rules! local_mac_tt {
    ($tt:tt) => { let $tt = 42; }
}

fn main() {
    let x = 2.0.neg();
    //~^ ERROR can't call method `neg` on ambiguous numeric type `{float}`

    let y = 2.0;
    let x = y.neg();
    //~^ ERROR can't call method `neg` on ambiguous numeric type `{float}`
    println!("{:?}", x);

    for i in 0..100 {
        println!("{}", i.pow(2));
        //~^ ERROR can't call method `pow` on ambiguous numeric type `{integer}`
    }

    local_mac!(local_bar);
    local_bar.pow(2);
    //~^ ERROR can't call method `pow` on ambiguous numeric type `{integer}`

    local_mac_tt!(local_bar_tt);
    local_bar_tt.pow(2);
    //~^ ERROR can't call method `pow` on ambiguous numeric type `{integer}`
}

fn qux() {
    mac!(bar);
    bar.pow(2);
    //~^ ERROR can't call method `pow` on ambiguous numeric type `{integer}`
}
"#;

    let aux_source = r#"#[macro_export]
macro_rules! mac {
    ($ident:ident) => { let $ident = 42; }
}

#[macro_export]
macro_rules! inline {
    () => ()
}
"#;
    let input = Level::ERROR
        .message("can't call method `pow` on ambiguous numeric type `{integer}`")
        .id("E0689")
        .group(
            Group::new().element(
                Snippet::source(source)
                    .line_start(1)
                    .origin("$DIR/method-on-ambiguous-numeric-type.rs")
                    .fold(true)
                    .annotation(AnnotationKind::Primary.span(916..919)),
            ),
        )
        .group(
            Group::new()
                .element(Level::HELP.title("you must specify a type for this binding, like `i32`"))
                .element(
                    Snippet::source(aux_source)
                        .line_start(1)
                        .origin("$DIR/auxiliary/macro-in-other-crate.rs")
                        .fold(true)
                        .annotation(AnnotationKind::Context.span(69..69).label(": i32")),
                ),
        );
    let expected = str![[r#"
error[E0689]: can't call method `pow` on ambiguous numeric type `{integer}`
  --> $DIR/method-on-ambiguous-numeric-type.rs:37:9
   |
LL |     bar.pow(2);
   |         ^^^
   |
help: you must specify a type for this binding, like `i32`
  --> $DIR/auxiliary/macro-in-other-crate.rs:3:35
   |
LL |     ($ident:ident) => { let $ident = 42; }
   |                                   - : i32
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn issue_42234_unknown_receiver_type() {
    // tests/ui/span/issue-42234-unknown-receiver-type.rs
    let source = r#"//@ revisions: full generic_arg
#![cfg_attr(generic_arg, feature(generic_arg_infer))]

// When the type of a method call's receiver is unknown, the span should point
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

    let input = Level::ERROR
        .message("type annotations needed")
        .id("E0282")
        .group(
            Group::new().element(
                Snippet::source(source)
                    .line_start(1)
                    .origin("$DIR/issue-42234-unknown-receiver-type.rs")
                    .fold(true)
                    .annotation(AnnotationKind::Primary.span(536..539).label(
                        "cannot infer type of the type parameter `S` declared on the method `sum`",
                    )),
            ),
        );
    let expected = str![[r#"
error[E0282]: type annotations needed
  --> $DIR/issue-42234-unknown-receiver-type.rs:15:10
   |
LL |         .sum::<_>() //~ ERROR type annotations needed
   |          ^^^ cannot infer type of the type parameter `S` declared on the method `sum`
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn pattern_usefulness_empty_match() {
    // tests/ui/pattern/usefulness/empty-match.rs
    let source = r##"//@ revisions: normal exhaustive_patterns
//
// This tests a match with no arms on various types.
#![feature(never_type)]
#![cfg_attr(exhaustive_patterns, feature(exhaustive_patterns))]
#![deny(unreachable_patterns)]

fn nonempty<const N: usize>(arrayN_of_empty: [!; N]) {
    macro_rules! match_no_arms {
        ($e:expr) => {
            match $e {}
        };
    }
    macro_rules! match_guarded_arm {
        ($e:expr) => {
            match $e {
                _ if false => {}
            }
        };
    }

    struct NonEmptyStruct1;
    struct NonEmptyStruct2(bool);
    union NonEmptyUnion1 {
        foo: (),
    }
    union NonEmptyUnion2 {
        foo: (),
        bar: !,
    }
    enum NonEmptyEnum1 {
        Foo(bool),
    }
    enum NonEmptyEnum2 {
        Foo(bool),
        Bar,
    }
    enum NonEmptyEnum5 {
        V1,
        V2,
        V3,
        V4,
        V5,
    }
    let array0_of_empty: [!; 0] = [];

    match_no_arms!(0u8); //~ ERROR type `u8` is non-empty
    match_no_arms!(0i8); //~ ERROR type `i8` is non-empty
    match_no_arms!(0usize); //~ ERROR type `usize` is non-empty
    match_no_arms!(0isize); //~ ERROR type `isize` is non-empty
    match_no_arms!(NonEmptyStruct1); //~ ERROR type `NonEmptyStruct1` is non-empty
    match_no_arms!(NonEmptyStruct2(true)); //~ ERROR type `NonEmptyStruct2` is non-empty
    match_no_arms!((NonEmptyUnion1 { foo: () })); //~ ERROR type `NonEmptyUnion1` is non-empty
    match_no_arms!((NonEmptyUnion2 { foo: () })); //~ ERROR type `NonEmptyUnion2` is non-empty
    match_no_arms!(NonEmptyEnum1::Foo(true)); //~ ERROR `NonEmptyEnum1::Foo(_)` not covered
    match_no_arms!(NonEmptyEnum2::Foo(true)); //~ ERROR `NonEmptyEnum2::Foo(_)` and `NonEmptyEnum2::Bar` not covered
    match_no_arms!(NonEmptyEnum5::V1); //~ ERROR `NonEmptyEnum5::V1`, `NonEmptyEnum5::V2`, `NonEmptyEnum5::V3` and 2 more not covered
    match_no_arms!(array0_of_empty); //~ ERROR type `[!; 0]` is non-empty
    match_no_arms!(arrayN_of_empty); //~ ERROR type `[!; N]` is non-empty

    match_guarded_arm!(0u8); //~ ERROR `0_u8..=u8::MAX` not covered
    match_guarded_arm!(0i8); //~ ERROR `i8::MIN..=i8::MAX` not covered
    match_guarded_arm!(0usize); //~ ERROR `0_usize..` not covered
    match_guarded_arm!(0isize); //~ ERROR `_` not covered
    match_guarded_arm!(NonEmptyStruct1); //~ ERROR `NonEmptyStruct1` not covered
    match_guarded_arm!(NonEmptyStruct2(true)); //~ ERROR `NonEmptyStruct2(_)` not covered
    match_guarded_arm!((NonEmptyUnion1 { foo: () })); //~ ERROR `NonEmptyUnion1 { .. }` not covered
    match_guarded_arm!((NonEmptyUnion2 { foo: () })); //~ ERROR `NonEmptyUnion2 { .. }` not covered
    match_guarded_arm!(NonEmptyEnum1::Foo(true)); //~ ERROR `NonEmptyEnum1::Foo(_)` not covered
    match_guarded_arm!(NonEmptyEnum2::Foo(true)); //~ ERROR `NonEmptyEnum2::Foo(_)` and `NonEmptyEnum2::Bar` not covered
    match_guarded_arm!(NonEmptyEnum5::V1); //~ ERROR `NonEmptyEnum5::V1`, `NonEmptyEnum5::V2`, `NonEmptyEnum5::V3` and 2 more not covered
    match_guarded_arm!(array0_of_empty); //~ ERROR `[]` not covered
    match_guarded_arm!(arrayN_of_empty); //~ ERROR `[]` not covered
}

fn main() {}
"##;

    let input = Level::ERROR
        .message(
            "non-exhaustive patterns: `NonEmptyEnum5::V1`, `NonEmptyEnum5::V2`, `NonEmptyEnum5::V3` and 2 more not covered"
        )
        .id("E0004")
        .group(
            Group::new().element(
                Snippet::source(source)
                    .line_start(1)
                    .origin("$DIR/empty-match.rs")
                    .fold(true)
                    .annotation(
                        AnnotationKind::Primary
                            .span(2911..2928)
                            .label("patterns `NonEmptyEnum5::V1`, `NonEmptyEnum5::V2`, `NonEmptyEnum5::V3` and 2 more not covered")
                    ),
            ),
        )
        .group(
            Group::new()
                .element(Level::NOTE.title("`NonEmptyEnum5` defined here"))
                .element(
                    Snippet::source(source)
                        .line_start(1)
                        .origin("$DIR/empty-match.rs")
                        .fold(true)
                        .annotation(AnnotationKind::Primary.span(818..831))
                        .annotation(AnnotationKind::Context.span(842..844).label("not covered"))
                        .annotation(AnnotationKind::Context.span(854..856).label("not covered"))
                        .annotation(AnnotationKind::Context.span(866..868).label("not covered"))
                        .annotation(AnnotationKind::Context.span(878..880).label("not covered"))
                        .annotation(AnnotationKind::Context.span(890..892).label("not covered"))
                )
                .element(Level::NOTE.title("the matched value is of type `NonEmptyEnum5`"))
                .element(Level::NOTE.title("match arms with guards don't count towards exhaustivity"))
        )
        .group(
            Group::new()
                .element(
                    Level::HELP
                        .title("ensure that all possible cases are being handled by adding a match arm with a wildcard pattern as shown, or multiple match arms")
                )
                .element(
                    Snippet::source(source)
                        .line_start(1)
                        .origin("$DIR/empty-match.rs")
                        .fold(true)
                        .annotation(AnnotationKind::Context.span(485..485).label(",\n                _ => todo!()"))
                )
        );
    let expected = str![[r#"
error[E0004]: non-exhaustive patterns: `NonEmptyEnum5::V1`, `NonEmptyEnum5::V2`, `NonEmptyEnum5::V3` and 2 more not covered
  --> $DIR/empty-match.rs:71:24
   |
LL |     match_guarded_arm!(NonEmptyEnum5::V1); //~ ERROR `NonEmptyEnum5::V1`, `NonEmptyEnum5::V2`, `NonEmptyEnum5::V3` and 2 more not covered
   |                        ^^^^^^^^^^^^^^^^^ patterns `NonEmptyEnum5::V1`, `NonEmptyEnum5::V2`, `NonEmptyEnum5::V3` and 2 more not covered
   |
note: `NonEmptyEnum5` defined here
  --> $DIR/empty-match.rs:38:10
   |
LL |     enum NonEmptyEnum5 {
   |          ^^^^^^^^^^^^^
LL |         V1,
   |         -- not covered
LL |         V2,
   |         -- not covered
LL |         V3,
   |         -- not covered
LL |         V4,
   |         -- not covered
LL |         V5,
   |         -- not covered
   = note: the matched value is of type `NonEmptyEnum5`
   = note: match arms with guards don't count towards exhaustivity
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern as shown, or multiple match arms
  --> $DIR/empty-match.rs:17:33
   |
LL |                 _ if false => {}
   |                                 - ,
                _ => todo!()
"#]];
    let renderer = Renderer::plain()
        .anonymized_line_numbers(true)
        .term_width(annotate_snippets::renderer::DEFAULT_TERM_WIDTH + 4);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn object_fail() {
    // tests/ui/traits/alias/object-fail.rs
    let source = r#"#![feature(trait_alias)]

trait EqAlias = Eq;
trait IteratorAlias = Iterator;

fn main() {
    let _: &dyn EqAlias = &123;
    //~^ ERROR the trait alias `EqAlias` is not dyn compatible [E0038]
    let _: &dyn IteratorAlias = &vec![123].into_iter();
    //~^ ERROR must be specified
}
"#;
    let input = Level::ERROR
        .message("the trait alias `EqAlias` is not dyn compatible")
        .id("E0038")
        .group(
            Group::new().element(
                Snippet::source(source)
                    .line_start(1)
                    .origin("$DIR/object-fail.rs")
                    .fold(true)
                    .annotation(
                        AnnotationKind::Primary
                            .span(107..114)
                            .label("`EqAlias` is not dyn compatible"),
                    ),
            ),
        )
        .group(
            Group::new()
                .element(
                    Level::NOTE
                        .title("for a trait to be dyn compatible it needs to allow building a vtable\nfor more information, visit <https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility>"))
                .element(
                    Origin::new("$SRC_DIR/core/src/cmp.rs")
                        .line(334)
                        .char_column(14)
                        .primary(true)
                        .label("...because it uses `Self` as a type parameter")

                )
                .element(
                    Snippet::source(source)
                        .line_start(1)
                        .origin("$DIR/object-fail.rs")
                        .fold(true)
                        .annotation(
                            AnnotationKind::Context
                                .span(32..39)
                                .label("this trait is not dyn compatible..."),
                        ),
                ),
        );
    let expected = str![[r#"
error[E0038]: the trait alias `EqAlias` is not dyn compatible
  --> $DIR/object-fail.rs:7:17
   |
LL |     let _: &dyn EqAlias = &123;
   |                 ^^^^^^^ `EqAlias` is not dyn compatible
   |
note: for a trait to be dyn compatible it needs to allow building a vtable
      for more information, visit <https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility>
  --> $SRC_DIR/core/src/cmp.rs:334:14
   |
   = note: ...because it uses `Self` as a type parameter
   |
  ::: $DIR/object-fail.rs:3:7
   |
LL | trait EqAlias = Eq;
   |       ------- this trait is not dyn compatible...
"#]];

    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input), expected);
}
