//! These tests have been adapted from [Rust's parser tests][parser-tests].
//!
//! [parser-tests]: https://github.com/rust-lang/rust/blob/894f7a4ba6554d3797404bbf550d9919df060b97/compiler/rustc_parse/src/parser/tests.rs

use annotate_snippets::{AnnotationKind, Group, Level, Origin, Padding, Patch, Renderer, Snippet};

use annotate_snippets::renderer::OutputTheme;
use snapbox::{assert_data_eq, str};

#[test]
fn ends_on_col0() {
    let source = r#"
fn foo() {
}
"#;
    let input = &[Group::new().element(Level::ERROR.title("foo")).element(
        Snippet::source(source)
            .line_start(1)
            .path("test.rs")
            .fold(true)
            .annotation(AnnotationKind::Primary.span(10..13).label("test")),
    )];

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
    let input = &[Group::new().element(Level::ERROR.title("foo")).element(
        Snippet::source(source)
            .line_start(1)
            .path("test.rs")
            .fold(true)
            .annotation(AnnotationKind::Primary.span(10..17).label("test")),
    )];
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
    let input = &[Group::new().element(Level::ERROR.title("foo")).element(
        Snippet::source(source)
            .line_start(1)
            .path("test.rs")
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
    )];

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
    let input = &[Group::new().element(Level::ERROR.title("foo")).element(
        Snippet::source(source)
            .line_start(1)
            .path("test.rs")
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
    )];

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
    let input = &[Group::new().element(Level::ERROR.title("foo")).element(
        Snippet::source(source)
            .line_start(1)
            .path("test.rs")
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
    )];

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
    let input = &[Group::new().element(Level::ERROR.title("foo")).element(
        Snippet::source(source)
            .line_start(1)
            .path("test.rs")
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
    )];

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
    let input = &[Group::new().element(Level::ERROR.title("foo")).element(
        Snippet::source(source)
            .line_start(1)
            .path("test.rs")
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
    )];

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
    let input = &[Group::new().element(Level::ERROR.title("foo")).element(
        Snippet::source(source)
            .line_start(1)
            .path("test.rs")
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
    )];

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
    let input = &[Group::new().element(Level::ERROR.title("foo")).element(
        Snippet::source(source)
            .line_start(1)
            .path("test.rs")
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
    )];

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
    let input = &[Group::new().element(Level::ERROR.title("foo")).element(
        Snippet::source(source)
            .line_start(1)
            .path("test.rs")
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
    )];

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
    let input = &[Group::new().element(Level::ERROR.title("foo")).element(
        Snippet::source(source)
            .line_start(1)
            .path("test.rs")
            .fold(true)
            .annotation(AnnotationKind::Primary.span(18..25).label(""))
            .annotation(
                AnnotationKind::Context
                    .span(14..27)
                    .label("`a` is a good letter"),
            )
            .annotation(AnnotationKind::Context.span(22..23).label("")),
    )];

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
    let input = &[Group::new().element(Level::ERROR.title("foo")).element(
        Snippet::source(source)
            .line_start(1)
            .path("test.rs")
            .fold(true)
            .annotation(
                AnnotationKind::Primary
                    .span(14..27)
                    .label("`a` is a good letter"),
            )
            .annotation(AnnotationKind::Context.span(18..25).label("")),
    )];

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
    let input = &[Group::new().element(Level::ERROR.title("foo")).element(
        Snippet::source(source)
            .line_start(1)
            .path("test.rs")
            .fold(true)
            .annotation(
                AnnotationKind::Primary
                    .span(18..25)
                    .label("`b` is a good letter"),
            )
            .annotation(AnnotationKind::Context.span(14..27).label(""))
            .annotation(AnnotationKind::Context.span(22..23).label("")),
    )];

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
    let input = &[Group::new().element(Level::ERROR.title("foo")).element(
        Snippet::source(source)
            .line_start(1)
            .path("test.rs")
            .fold(true)
            .annotation(AnnotationKind::Primary.span(14..27).label(""))
            .annotation(
                AnnotationKind::Context
                    .span(18..25)
                    .label("`b` is a good letter"),
            ),
    )];

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
    let input = &[Group::new().element(Level::ERROR.title("foo")).element(
        Snippet::source(source)
            .line_start(1)
            .path("test.rs")
            .fold(true)
            .annotation(
                AnnotationKind::Primary
                    .span(14..18)
                    .label("`a` is a good letter"),
            )
            .annotation(AnnotationKind::Context.span(18..22).label("")),
    )];

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
    let input = &[Group::new().element(Level::ERROR.title("foo")).element(
        Snippet::source(source)
            .line_start(1)
            .path("test.rs")
            .fold(true)
            .annotation(AnnotationKind::Primary.span(14..27).label(""))
            .annotation(AnnotationKind::Context.span(18..25).label("")),
    )];

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
    let input = &[Group::new().element(Level::ERROR.title("foo")).element(
        Snippet::source(source)
            .line_start(1)
            .path("test.rs")
            .fold(true)
            .annotation(AnnotationKind::Primary.span(18..25).label(""))
            .annotation(AnnotationKind::Context.span(14..27).label(""))
            .annotation(AnnotationKind::Context.span(22..23).label("")),
    )];

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
    let input = &[Group::new().element(Level::ERROR.title("foo")).element(
        Snippet::source(source)
            .line_start(1)
            .path("test.rs")
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
    )];

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
    let input = &[Group::new().element(Level::ERROR.title("foo")).element(
        Snippet::source(source)
            .line_start(1)
            .path("test.rs")
            .fold(true)
            .annotation(
                AnnotationKind::Primary
                    .span(14..27)
                    .label("`a` is a good letter"),
            ),
    )];

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
    let input = &[Group::new().element(Level::ERROR.title("foo")).element(
        Snippet::source(source)
            .line_start(1)
            .path("test.rs")
            .fold(true)
            .annotation(AnnotationKind::Primary.span(14..27).label("")),
    )];

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
    let input = &[Group::new().element(Level::ERROR.title("foo")).element(
        Snippet::source(source)
            .line_start(1)
            .path("test.rs")
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
    )];

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
    let input = &[Group::new().element(Level::ERROR.title("foo")).element(
        Snippet::source(source)
            .line_start(1)
            .path("test.rs")
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
    )];

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
    let input = &[Group::new()
        .element(Level::ERROR.title("this file contains an unclosed delimiter"))
        .element(
            Snippet::source(source)
                .line_start(1)
                .path("$DIR/issue-91334.rs")
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
        )];
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
    let input = &[
        Group::new()
            .element(
                Level::ERROR
                    .title("`break` with value from a `while` loop")
                    .id("E0571"),
            )
            .element(
                Snippet::source(source)
                    .line_start(1)
                    .path("$DIR/issue-114529-illegal-break-with-value.rs")
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
        Group::new()
            .element(
                Level::HELP
                    .title("use `break` on its own without a value inside this `while` loop"),
            )
            .element(
                Snippet::source(source)
                    .line_start(1)
                    .path("$DIR/issue-114529-illegal-break-with-value.rs")
                    .fold(true)
                    .annotation(AnnotationKind::Context.span(483..581).label("break")),
            ),
    ];
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
        &[
            Group::new()
                .element(
                    Level::ERROR
                        .title("`V0usize` cannot be safely transmuted into `[usize; 2]`")
                        .id("E0277"),
                )
                .element(
                    Snippet::source(source)
                        .line_start(1)
                        .path("$DIR/primitive_reprs_should_have_correct_length.rs")
                        .fold(true)
                        .annotation(AnnotationKind::Primary.span(4375..4381).label(
                            "the size of `V0usize` is smaller than the size of `[usize; 2]`",
                        )),
                ),
            Group::new()
                .element(Level::NOTE.title("required by a bound in `is_transmutable`"))
                .element(
                    Snippet::source(source)
                        .line_start(1)
                        .path("$DIR/primitive_reprs_should_have_correct_length.rs")
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
        ];
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
    let input = &[Group::new().element(Level::ERROR
        .title("`&[u8; 0]` cannot be safely transmuted into `&[u16; 0]`")
        .id("E027s7")).element(
                Snippet::source(source)
                    .line_start(1)
                    .fold(true)
                    .path("$DIR/align-fail.rs")
                    .annotation(
                        AnnotationKind::Primary
                            .span(442..459)
                            .label("the minimum alignment of `&[u8; 0]` (1) should be greater than that of `&[u16; 0]` (2)")
                    ),
            )];
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
    let input =
        &[Group::new()
            .element(
                Level::ERROR
                    .title("expected function, found `{integer}`")
                    .id("E0618"),
            )
            .element(
                Snippet::source(source)
                    .line_start(1)
                    .path("$DIR/missing-semicolon.rs")
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
            )];
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
    let input =
           &[ Group::new().element(Level::WARNING
                .title("non-local `macro_rules!` definition, `#[macro_export]` macro should be written at top level module"))
                .element(
                    Snippet::source(aux_source)
                        .line_start(1)
                        .path("$DIR/auxiliary/nested-macro-rules.rs")
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
                        .path("$DIR/nested-macro-rules.rs")
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
            Group::new()
                .element(Level::NOTE.title("the lint level is defined here"))
                .element(
                    Snippet::source(source)
                        .line_start(1)
                        .path("$DIR/nested-macro-rules.rs")
                        .fold(true)
                        .annotation(AnnotationKind::Primary.span(224..245)),
                )];
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
    let input = &[
        Group::new()
            .element(
                Level::ERROR
                    .title("can't call method `pow` on ambiguous numeric type `{integer}`")
                    .id("E0689"),
            )
            .element(
                Snippet::source(source)
                    .line_start(1)
                    .path("$DIR/method-on-ambiguous-numeric-type.rs")
                    .fold(true)
                    .annotation(AnnotationKind::Primary.span(916..919)),
            ),
        Group::new()
            .element(Level::HELP.title("you must specify a type for this binding, like `i32`"))
            .element(
                Snippet::source(aux_source)
                    .line_start(1)
                    .path("$DIR/auxiliary/macro-in-other-crate.rs")
                    .fold(true)
                    .annotation(AnnotationKind::Context.span(69..69).label(": i32")),
            ),
    ];
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

    let input = &[Group::new()
        .element(Level::ERROR.title("type annotations needed").id("E0282"))
        .element(
            Snippet::source(source)
                .line_start(1)
                .path("$DIR/issue-42234-unknown-receiver-type.rs")
                .fold(true)
                .annotation(AnnotationKind::Primary.span(536..539).label(
                    "cannot infer type of the type parameter `S` declared on the method `sum`",
                )),
        )];
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

    let input =
           &[ Group::new().element( Level::ERROR
                .title(
                    "non-exhaustive patterns: `NonEmptyEnum5::V1`, `NonEmptyEnum5::V2`, `NonEmptyEnum5::V3` and 2 more not covered"
                )
                .id("E0004")).element(
                Snippet::source(source)
                    .line_start(1)
                    .path("$DIR/empty-match.rs")
                    .fold(true)
                    .annotation(
                        AnnotationKind::Primary
                            .span(2911..2928)
                            .label("patterns `NonEmptyEnum5::V1`, `NonEmptyEnum5::V2`, `NonEmptyEnum5::V3` and 2 more not covered")
                    ),
            ),
            Group::new()
                .element(Level::NOTE.title("`NonEmptyEnum5` defined here"))
                .element(
                    Snippet::source(source)
                        .line_start(1)
                        .path("$DIR/empty-match.rs")
                        .fold(true)
                        .annotation(AnnotationKind::Primary.span(818..831))
                        .annotation(AnnotationKind::Context.span(842..844).label("not covered"))
                        .annotation(AnnotationKind::Context.span(854..856).label("not covered"))
                        .annotation(AnnotationKind::Context.span(866..868).label("not covered"))
                        .annotation(AnnotationKind::Context.span(878..880).label("not covered"))
                        .annotation(AnnotationKind::Context.span(890..892).label("not covered"))
                )
                .element(Level::NOTE.title("the matched value is of type `NonEmptyEnum5`"))
                .element(Level::NOTE.title("match arms with guards don't count towards exhaustivity")
        ),
            Group::new()
                .element(
                    Level::HELP
                        .title("ensure that all possible cases are being handled by adding a match arm with a wildcard pattern as shown, or multiple match arms")
                )
                .element(
                    Snippet::source(source)
                        .line_start(1)
                        .path("$DIR/empty-match.rs")
                        .fold(true)
                        .annotation(AnnotationKind::Context.span(485..485).label(",\n                _ => todo!()"))

        )];
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
    let input = &[Group::new().element(Level::ERROR
        .title("the trait alias `EqAlias` is not dyn compatible")
        .id("E0038")).element(
                Snippet::source(source)
                    .line_start(1)
                    .path("$DIR/object-fail.rs")
                    .fold(true)
                    .annotation(
                        AnnotationKind::Primary
                            .span(107..114)
                            .label("`EqAlias` is not dyn compatible"),
                    ),
            ),
            Group::new()
                .element(
                    Level::NOTE
                        .title("for a trait to be dyn compatible it needs to allow building a vtable\nfor more information, visit <https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility>"))
                .element(
                    Origin::new("$SRC_DIR/core/src/cmp.rs")
                        .line(334)
                        .char_column(14)
                        .primary(true)
                )
                .element(Padding)
                .element(Level::NOTE.title("...because it uses `Self` as a type parameter"))
                .element(
                    Snippet::source(source)
                        .line_start(1)
                        .path("$DIR/object-fail.rs")
                        .fold(true)
                        .annotation(
                            AnnotationKind::Context
                                .span(32..39)
                                .label("this trait is not dyn compatible..."),
                        ),
                )];
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

#[test]
fn long_span_shortest() {
    // tests/ui/diagnostic-width/long-span.rs
    let source = r#"
const C: u8 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

fn main() {}
"#;
    let input = &[Group::new()
        .element(Level::ERROR.title("mismatched types").id("E0038"))
        .element(
            Snippet::source(source)
                .path("$DIR/long-span.rs")
                .fold(true)
                .annotation(
                    AnnotationKind::Primary
                        .span(15..5055)
                        .label("expected `u8`, found `[{integer}; 1680]`"),
                ),
        )];
    let expected = str![[r#"
error[E0038]: mismatched types
  --> $DIR/long-span.rs:2:15
   |
LL | ... = [0, 0, 0...0];
   |       ^^^^^^^^...^^ expected `u8`, found `[{integer}; 1680]`
"#]];

    let renderer = Renderer::plain()
        .anonymized_line_numbers(true)
        .term_width(8);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn long_span_short() {
    // tests/ui/diagnostic-width/long-span.rs
    let source = r#"
const C: u8 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

fn main() {}
"#;
    let input = &[Group::new()
        .element(Level::ERROR.title("mismatched types").id("E0038"))
        .element(
            Snippet::source(source)
                .path("$DIR/long-span.rs")
                .fold(true)
                .annotation(
                    AnnotationKind::Primary
                        .span(15..5055)
                        .label("expected `u8`, found `[{integer}; 1680]`"),
                ),
        )];
    let expected = str![[r#"
error[E0038]: mismatched types
   ╭▸ $DIR/long-span.rs:2:15
   │
LL │ …u8 = [0, 0, 0…0];
   ╰╴      ━━━━━━━━…━━ expected `u8`, found `[{integer}; 1680]`
"#]];

    let renderer = Renderer::plain()
        .anonymized_line_numbers(true)
        .term_width(12)
        .theme(OutputTheme::Unicode);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn long_span_long() {
    // tests/ui/diagnostic-width/long-span.rs
    let source = r#"
const C: u8 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

fn main() {}
"#;
    let input = &[Group::new()
        .element(Level::ERROR.title("mismatched types").id("E0038"))
        .element(
            Snippet::source(source)
                .path("$DIR/long-span.rs")
                .fold(true)
                .annotation(
                    AnnotationKind::Primary
                        .span(15..5055)
                        .label("expected `u8`, found `[{integer}; 1680]`"),
                ),
        )];
    let expected = str![[r#"
error[E0038]: mismatched types
   ╭▸ $DIR/long-span.rs:2:15
   │
LL │ …u8 = [0, 0, 0, 0, 0, 0, 0, 0, 0, …, 0, 0, 0, 0, 0, 0, 0];
   ╰╴      ━━━━━━━━━━━━━━━━━━━━━━━━━━━━…━━━━━━━━━━━━━━━━━━━━━━ expected `u8`, found `[{integer}; 1680]`
"#]];

    let renderer = Renderer::plain()
        .anonymized_line_numbers(true)
        .term_width(80)
        .theme(OutputTheme::Unicode);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn long_span_longest() {
    // tests/ui/diagnostic-width/long-span.rs
    let source = r#"
const C: u8 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

fn main() {}
"#;
    let input = &[Group::new()
        .element(Level::ERROR.title("mismatched types").id("E0038"))
        .element(
            Snippet::source(source)
                .path("$DIR/long-span.rs")
                .fold(true)
                .annotation(
                    AnnotationKind::Primary
                        .span(15..5055)
                        .label("expected `u8`, found `[{integer}; 1680]`"),
                ),
        )];
    let expected = str![[r#"
error[E0038]: mismatched types
  --> $DIR/long-span.rs:2:15
   |
LL | ... = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0...0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
   |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^...^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `u8`, found `[{integer}; 1680]`
"#]];

    let renderer = Renderer::plain()
        .anonymized_line_numbers(true)
        .term_width(120);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn lint_map_unit_fn() {
    // tests/ui/lint/lint_map_unit_fn.rs
    let source = r#"#![deny(map_unit_fn)]

fn foo(items: &mut Vec<u8>) {
    items.sort();
}

fn main() {
    let mut x: Vec<Vec<u8>> = vec![vec![0, 2, 1], vec![5, 4, 3]];
    x.iter_mut().map(foo);
    //~^ ERROR `Iterator::map` call that discard the iterator's values
    x.iter_mut().map(|items| {
    //~^ ERROR `Iterator::map` call that discard the iterator's values
        items.sort();
    });
    let f = |items: &mut Vec<u8>| {
        items.sort();
    };
    x.iter_mut().map(f);
    //~^ ERROR `Iterator::map` call that discard the iterator's values
}
"#;

    let input = &[Group::new().element(Level::ERROR
        .title("`Iterator::map` call that discard the iterator's values"))
                .element(
                    Snippet::source(source)
                        .path("$DIR/lint_map_unit_fn.rs")
                        .fold(true)
                        .annotation(AnnotationKind::Context.span(271..278).label(
                            "this function returns `()`, which is likely not what you wanted",
                        ))
                        .annotation(
                            AnnotationKind::Context
                                .span(271..379)
                                .label("called `Iterator::map` with callable that returns `()`"),
                        )
                        .annotation(
                            AnnotationKind::Context
                                .span(267..380)
                                .label("after this call to map, the resulting iterator is `impl Iterator<Item = ()>`, which means the only information carried by the iterator is the number of items")
                        )
                        .annotation(AnnotationKind::Primary.span(267..380)),
                )
                .element(
                    Level::NOTE.title("`Iterator::map`, like many of the methods on `Iterator`, gets executed lazily, meaning that its effects won't be visible until it is iterated")),
            Group::new()
                .element(Level::HELP.title("you might have meant to use `Iterator::for_each`"))
                .element(
                    Snippet::source(source)
                        .path("$DIR/lint_map_unit_fn.rs")
                        .fold(true)
                        .patch(Patch::new(267..270, r#"for_each"#)),
                )];

    let expected = str![[r#"
error: `Iterator::map` call that discard the iterator's values
  --> $DIR/lint_map_unit_fn.rs:11:18
   |
LL |         x.iter_mut().map(|items| {
   |                      ^   -------
   |                      |   |
   |  ____________________|___this function returns `()`, which is likely not what you wanted
   | |  __________________|
   | | |
LL | | |     //~^ ERROR `Iterator::map` call that discard the iterator's values
LL | | |         items.sort();
LL | | |     });
   | | |     -^ after this call to map, the resulting iterator is `impl Iterator<Item = ()>`, which means the only information carried by the iterator is the number of items
   | | |_____||
   | |_______|
   |         called `Iterator::map` with callable that returns `()`
   |
   = note: `Iterator::map`, like many of the methods on `Iterator`, gets executed lazily, meaning that its effects won't be visible until it is iterated
help: you might have meant to use `Iterator::for_each`
   |
LL -     x.iter_mut().map(|items| {
LL +     x.iter_mut().for_each(|items| {
   |
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn bad_char_literals() {
    // tests/ui/parser/bad-char-literals.rs

    let source = r#"// ignore-tidy-cr
// ignore-tidy-tab

fn main() {
    // these literals are just silly.
    ''';
    //~^ ERROR: character constant must be escaped: `'`

    // note that this is a literal "\n" byte
    '
';
    //~^^ ERROR: character constant must be escaped: `\n`

    // note that this is a literal "\r" byte
; //~ ERROR: character constant must be escaped: `\r`

    // note that this is a literal NULL
    '--'; //~ ERROR: character literal may only contain one codepoint

    // note that this is a literal tab character here
    '  ';
    //~^ ERROR: character constant must be escaped: `\t`
}
"#;

    let input = &[
        Group::new()
            .element(Level::ERROR.title("character constant must be escaped: `\\n`"))
            .element(
                Snippet::source(source)
                    .path("$DIR/bad-char-literals.rs")
                    .fold(true)
                    .annotation(AnnotationKind::Primary.span(204..205)),
            ),
        Group::new()
            .element(Level::HELP.title("escape the character"))
            .element(
                Snippet::source(source)
                    .path("$DIR/bad-char-literals.rs")
                    .line_start(1)
                    .fold(true)
                    .patch(Patch::new(204..205, r#"\n"#)),
            ),
    ];
    let expected = str![[r#"
error: character constant must be escaped: `/n`
  --> $DIR/bad-char-literals.rs:10:6
   |
LL |       '
   |  ______^
LL | | ';
   | |_^
   |
help: escape the character
   |
LL |     '/n';
   |      ++
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn unclosed_1() {
    // tests/ui/frontmatter/unclosed-1.rs

    let source = r#"----cargo
//~^ ERROR: unclosed frontmatter

// This test checks that the #! characters can help us recover a frontmatter
// close. There should not be a "missing `main` function" error as the rest
// are properly parsed.

#![feature(frontmatter)]

fn main() {}
"#;

    let input = &[
        Group::new()
            .element(Level::ERROR.title("unclosed frontmatter"))
            .element(
                Snippet::source(source)
                    .path("$DIR/unclosed-1.rs")
                    .fold(true)
                    .annotation(AnnotationKind::Primary.span(0..221)),
            ),
        Group::new()
            .element(Level::NOTE.title("frontmatter opening here was not closed"))
            .element(
                Snippet::source(source)
                    .path("$DIR/unclosed-1.rs")
                    .fold(true)
                    .annotation(AnnotationKind::Primary.span(0..4)),
            ),
    ];
    let expected = str![[r#"
error: unclosed frontmatter
  --> $DIR/unclosed-1.rs:1:1
   |
LL | / ----cargo
...  |
LL | |
   | |_^
   |
note: frontmatter opening here was not closed
  --> $DIR/unclosed-1.rs:1:1
   |
LL | ----cargo
   | ^^^^
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn unclosed_2() {
    // tests/ui/frontmatter/unclosed-2.rs

    let source = r#"----cargo
//~^ ERROR: unclosed frontmatter
//~| ERROR: frontmatters are experimental

//@ compile-flags: --crate-type lib

// Leading whitespace on the feature line prevents recovery. However
// the dashes quoted will not be used for recovery and the entire file
// should be treated as within the frontmatter block.

 #![feature(frontmatter)]

fn foo() -> &str {
    "----"
}
"#;

    let input = &[
        Group::new()
            .element(Level::ERROR.title("unclosed frontmatter"))
            .element(
                Snippet::source(source)
                    .path("$DIR/unclosed-2.rs")
                    .fold(true)
                    .annotation(AnnotationKind::Primary.span(0..377)),
            ),
        Group::new()
            .element(Level::NOTE.title("frontmatter opening here was not closed"))
            .element(
                Snippet::source(source)
                    .path("$DIR/unclosed-2.rs")
                    .fold(true)
                    .annotation(AnnotationKind::Primary.span(0..4)),
            ),
    ];
    let expected = str![[r#"
error: unclosed frontmatter
  --> $DIR/unclosed-2.rs:1:1
   |
LL | / ----cargo
...  |
LL | |     "----"
LL | | }
   | |__^
   |
note: frontmatter opening here was not closed
  --> $DIR/unclosed-2.rs:1:1
   |
LL | ----cargo
   | ^^^^
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn unclosed_3() {
    // tests/ui/frontmatter/unclosed-3.rs

    let source = r#"----cargo
//~^ ERROR: frontmatter close does not match the opening

//@ compile-flags: --crate-type lib

// Unfortunate recovery situation. Not really preventable with improving the
// recovery strategy, but this type of code is rare enough already.

 #![feature(frontmatter)]

fn foo(x: i32) -> i32 {
    ---x
    //~^ ERROR: invalid preceding whitespace for frontmatter close
    //~| ERROR: extra characters after frontmatter close are not allowed
}
//~^ ERROR: unexpected closing delimiter: `}`
"#;

    let input = &[
        Group::new()
            .element(Level::ERROR.title("invalid preceding whitespace for frontmatter close"))
            .element(
                Snippet::source(source)
                    .path("$DIR/unclosed-3.rs")
                    .fold(true)
                    .annotation(AnnotationKind::Primary.span(302..310)),
            ),
        Group::new()
            .element(Level::NOTE.title("frontmatter close should not be preceded by whitespace"))
            .element(
                Snippet::source(source)
                    .path("$DIR/unclosed-3.rs")
                    .fold(true)
                    .annotation(AnnotationKind::Primary.span(302..306)),
            ),
    ];
    let expected = str![[r#"
error: invalid preceding whitespace for frontmatter close
  --> $DIR/unclosed-3.rs:12:1
   |
LL |     ---x
   | ^^^^^^^^
   |
note: frontmatter close should not be preceded by whitespace
  --> $DIR/unclosed-3.rs:12:1
   |
LL |     ---x
   | ^^^^
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn unclosed_4() {
    // tests/ui/frontmatter/unclosed-4.rs

    let source = r#"----cargo
//~^ ERROR: unclosed frontmatter

//! Similarly, a module-level content should allow for recovery as well (as
//! per unclosed-1.rs)

#![feature(frontmatter)]

fn main() {}
"#;

    let input = &[
        Group::new()
            .element(Level::ERROR.title("unclosed frontmatter"))
            .element(
                Snippet::source(source)
                    .path("$DIR/unclosed-4.rs")
                    .fold(true)
                    .annotation(AnnotationKind::Primary.span(0..43)),
            ),
        Group::new()
            .element(Level::NOTE.title("frontmatter opening here was not closed"))
            .element(
                Snippet::source(source)
                    .path("$DIR/unclosed-4.rs")
                    .fold(true)
                    .annotation(AnnotationKind::Primary.span(0..4)),
            ),
    ];
    let expected = str![[r#"
error: unclosed frontmatter
  --> $DIR/unclosed-4.rs:1:1
   |
LL | / ----cargo
LL | | //~^ ERROR: unclosed frontmatter
LL | |
   | |_^
   |
note: frontmatter opening here was not closed
  --> $DIR/unclosed-4.rs:1:1
   |
LL | ----cargo
   | ^^^^
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn unclosed_5() {
    // tests/ui/frontmatter/unclosed-5.rs

    let source = r#"----cargo
//~^ ERROR: unclosed frontmatter
//~| ERROR: frontmatters are experimental

// Similarly, a use statement should allow for recovery as well (as
// per unclosed-1.rs)

use std::env;

fn main() {}
"#;

    let input = &[
        Group::new()
            .element(Level::ERROR.title("unclosed frontmatter"))
            .element(
                Snippet::source(source)
                    .path("$DIR/unclosed-5.rs")
                    .fold(true)
                    .annotation(AnnotationKind::Primary.span(0..176)),
            ),
        Group::new()
            .element(Level::NOTE.title("frontmatter opening here was not closed"))
            .element(
                Snippet::source(source)
                    .path("$DIR/unclosed-5.rs")
                    .fold(true)
                    .annotation(AnnotationKind::Primary.span(0..4)),
            ),
    ];

    let expected = str![[r#"
error: unclosed frontmatter
  --> $DIR/unclosed-5.rs:1:1
   |
LL | / ----cargo
...  |
LL | |
   | |_^
   |
note: frontmatter opening here was not closed
  --> $DIR/unclosed-5.rs:1:1
   |
LL | ----cargo
   | ^^^^
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn pat_tuple_field_count_cross() {
    // tests/ui/pattern/pat-tuple-field-count-cross.stderr

    let source = r#"//@ aux-build:declarations-for-tuple-field-count-errors.rs

extern crate declarations_for_tuple_field_count_errors;

use declarations_for_tuple_field_count_errors::*;

fn main() {
    match Z0 {
        Z0() => {} //~ ERROR expected tuple struct or tuple variant, found unit struct `Z0`
        Z0(x) => {} //~ ERROR expected tuple struct or tuple variant, found unit struct `Z0`
    }
    match Z1() {
        Z1 => {} //~ ERROR match bindings cannot shadow tuple structs
        Z1(x) => {} //~ ERROR this pattern has 1 field, but the corresponding tuple struct has 0 fields
    }

    match S(1, 2, 3) {
        S() => {} //~ ERROR this pattern has 0 fields, but the corresponding tuple struct has 3 fields
        S(1) => {} //~ ERROR this pattern has 1 field, but the corresponding tuple struct has 3 fields
        S(xyz, abc) => {} //~ ERROR this pattern has 2 fields, but the corresponding tuple struct has 3 fields
        S(1, 2, 3, 4) => {} //~ ERROR this pattern has 4 fields, but the corresponding tuple struct has 3 fields
    }
    match M(1, 2, 3) {
        M() => {} //~ ERROR this pattern has 0 fields, but the corresponding tuple struct has 3 fields
        M(1) => {} //~ ERROR this pattern has 1 field, but the corresponding tuple struct has 3 fields
        M(xyz, abc) => {} //~ ERROR this pattern has 2 fields, but the corresponding tuple struct has 3 fields
        M(1, 2, 3, 4) => {} //~ ERROR this pattern has 4 fields, but the corresponding tuple struct has 3 fields
    }

    match E1::Z0 {
        E1::Z0() => {} //~ ERROR expected tuple struct or tuple variant, found unit variant `E1::Z0`
        E1::Z0(x) => {} //~ ERROR expected tuple struct or tuple variant, found unit variant `E1::Z0`
    }
    match E1::Z1() {
        E1::Z1 => {} //~ ERROR expected unit struct, unit variant or constant, found tuple variant `E1::Z1`
        E1::Z1(x) => {} //~ ERROR this pattern has 1 field, but the corresponding tuple variant has 0 fields
    }
    match E1::S(1, 2, 3) {
        E1::S() => {} //~ ERROR this pattern has 0 fields, but the corresponding tuple variant has 3 fields
        E1::S(1) => {} //~ ERROR this pattern has 1 field, but the corresponding tuple variant has 3 fields
        E1::S(xyz, abc) => {} //~ ERROR this pattern has 2 fields, but the corresponding tuple variant has 3 fields
        E1::S(1, 2, 3, 4) => {} //~ ERROR this pattern has 4 fields, but the corresponding tuple variant has 3 fields
    }

    match E2::S(1, 2, 3) {
        E2::S() => {} //~ ERROR this pattern has 0 fields, but the corresponding tuple variant has 3 fields
        E2::S(1) => {} //~ ERROR this pattern has 1 field, but the corresponding tuple variant has 3 fields
        E2::S(xyz, abc) => {} //~ ERROR this pattern has 2 fields, but the corresponding tuple variant has 3 fields
        E2::S(1, 2, 3, 4) => {} //~ ERROR this pattern has 4 fields, but the corresponding tuple variant has 3 fields
    }
    match E2::M(1, 2, 3) {
        E2::M() => {} //~ ERROR this pattern has 0 fields, but the corresponding tuple variant has 3 fields
        E2::M(1) => {} //~ ERROR this pattern has 1 field, but the corresponding tuple variant has 3 fields
        E2::M(xyz, abc) => {} //~ ERROR this pattern has 2 fields, but the corresponding tuple variant has 3 fields
        E2::M(1, 2, 3, 4) => {} //~ ERROR this pattern has 4 fields, but the corresponding tuple variant has 3 fields
    }
}
"#;
    let source1 = r#"pub struct Z0;
pub struct Z1();

pub struct S(pub u8, pub u8, pub u8);
pub struct M(
    pub u8,
    pub u8,
    pub u8,
);

pub enum E1 { Z0, Z1(), S(u8, u8, u8) }

pub enum E2 {
    S(u8, u8, u8),
    M(
        u8,
        u8,
        u8,
    ),
}
"#;

    let input = &[Group::new().element(Level::ERROR
        .title("expected unit struct, unit variant or constant, found tuple variant `E1::Z1`")
        .id(r#"E0532"#))
                .element(
                    Snippet::source(source)
                        .path("$DIR/pat-tuple-field-count-cross.rs")
                        .fold(true)
                        .annotation(AnnotationKind::Primary.span(1760..1766)),
                )
                .element(
                    Snippet::source(source1)
                        .path("$DIR/auxiliary/declarations-for-tuple-field-count-errors.rs")
                        .fold(true)
                        .annotation(
                            AnnotationKind::Context
                                .span(143..145)
                                .label("`E1::Z1` defined here"),
                        )
                        .annotation(
                            AnnotationKind::Context
                                .span(139..141)
                                .label("similarly named unit variant `Z0` defined here"),
                        ),
                ),
            Group::new()
                .element(Level::HELP.title("use the tuple variant pattern syntax instead"))
                .element(
                    Snippet::source(source)
                        .path("$DIR/pat-tuple-field-count-cross.rs")
                        .fold(true)
                        .patch(Patch::new(1760..1766, r#"E1::Z1()"#)),
                ),
            Group::new()
                .element(Level::HELP.title("a unit variant with a similar name exists"))
                .element(
                    Snippet::source(source)
                        .path("$DIR/pat-tuple-field-count-cross.rs")
                        .fold(true)
                        .patch(Patch::new(1764..1766, r#"Z0"#)),
                )];
    let expected = str![[r#"
error[E0532]: expected unit struct, unit variant or constant, found tuple variant `E1::Z1`
  --> $DIR/pat-tuple-field-count-cross.rs:35:9
   |
LL |         E1::Z1 => {} //~ ERROR expected unit struct, unit variant or constant, found tuple variant `E1::Z1`
   |         ^^^^^^
   |
  ::: $DIR/auxiliary/declarations-for-tuple-field-count-errors.rs:11:19
   |
LL | pub enum E1 { Z0, Z1(), S(u8, u8, u8) }
   |               --  -- `E1::Z1` defined here
   |               |
   |               similarly named unit variant `Z0` defined here
   |
help: use the tuple variant pattern syntax instead
   |
LL |         E1::Z1() => {} //~ ERROR expected unit struct, unit variant or constant, found tuple variant `E1::Z1`
   |               ++
help: a unit variant with a similar name exists
   |
LL -         E1::Z1 => {} //~ ERROR expected unit struct, unit variant or constant, found tuple variant `E1::Z1`
LL +         E1::Z0 => {} //~ ERROR expected unit struct, unit variant or constant, found tuple variant `E1::Z1`
   |
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn unterminated_nested_comment() {
    // tests/ui/lexer/unterminated-nested-comment.rs

    let source = r#"/* //~ ERROR E0758
/* */
/*
*/
"#;

    let input = &[Group::new()
        .element(Level::ERROR.title("unterminated block comment").id("E0758"))
        .element(
            Snippet::source(source)
                .path("$DIR/unterminated-nested-comment.rs")
                .fold(true)
                .annotation(
                    AnnotationKind::Context
                        .span(0..2)
                        .label("unterminated block comment"),
                )
                .annotation(AnnotationKind::Context.span(25..27).label(
                    "...as last nested comment starts here, maybe you want to close this instead?",
                ))
                .annotation(
                    AnnotationKind::Context
                        .span(28..30)
                        .label("...and last nested comment terminates here."),
                )
                .annotation(AnnotationKind::Primary.span(0..31)),
        )];

    let expected = str![[r#"
error[E0758]: unterminated block comment
  --> $DIR/unterminated-nested-comment.rs:1:1
   |
LL |   /* //~ ERROR E0758
   |   ^-
   |   |
   |  _unterminated block comment
   | |
LL | | /* */
LL | | /*
   | | --
   | | |
   | | ...as last nested comment starts here, maybe you want to close this instead?
LL | | */
   | |_--^
   |   |
   |   ...and last nested comment terminates here.
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn mismatched_types1() {
    // tests/ui/include-macros/mismatched-types.rs

    let file_txt_source = r#""#;

    let rust_source = r#"fn main() {
    let b: &[u8] = include_str!("file.txt");    //~ ERROR mismatched types
    let s: &str = include_bytes!("file.txt");   //~ ERROR mismatched types
}"#;

    let input = &[Group::new()
        .element(Level::ERROR.title("mismatched types").id("E0308"))
        .element(
            Snippet::source(file_txt_source)
                .fold(true)
                .line_start(3)
                .path("$DIR/file.txt")
                .annotation(
                    AnnotationKind::Primary
                        .span(0..0)
                        .label("expected `&[u8]`, found `&str`"),
                ),
        )
        .element(
            Snippet::source(rust_source)
                .path("$DIR/mismatched-types.rs")
                .fold(true)
                .annotation(
                    AnnotationKind::Context
                        .span(23..28)
                        .label("expected due to this"),
                )
                .annotation(
                    AnnotationKind::Context
                        .span(31..55)
                        .label("in this macro invocation"),
                ),
        )
        .element(
            Level::NOTE.title("expected reference `&[u8]`\n   found reference `&'static str`"),
        )];

    let expected = str![[r#"
error[E0308]: mismatched types
  --> $DIR/file.txt:3:1
   |
LL |
   | ^ expected `&[u8]`, found `&str`
   |
  ::: $DIR/mismatched-types.rs:2:12
   |
LL |     let b: &[u8] = include_str!("file.txt");    //~ ERROR mismatched types
   |            -----   ------------------------ in this macro invocation
   |            |
   |            expected due to this
   |
   = note: expected reference `&[u8]`
              found reference `&'static str`
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn mismatched_types2() {
    // tests/ui/include-macros/mismatched-types.rs

    let source = r#"fn main() {
    let b: &[u8] = include_str!("file.txt");    //~ ERROR mismatched types
    let s: &str = include_bytes!("file.txt");   //~ ERROR mismatched types
}"#;

    let input = &[Group::new()
        .element(Level::ERROR.title("mismatched types").id("E0308"))
        .element(
            Snippet::source(source)
                .path("$DIR/mismatched-types.rs")
                .fold(true)
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
            Level::NOTE.title("expected reference `&str`\n   found reference `&'static [u8; 0]`"),
        )];

    let expected = str![[r#"
error[E0308]: mismatched types
  --> $DIR/mismatched-types.rs:3:19
   |
LL |     let s: &str = include_bytes!("file.txt");   //~ ERROR mismatched types
   |            ----   ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&str`, found `&[u8; 0]`
   |            |
   |            expected due to this
   |
   = note: expected reference `&str`
              found reference `&'static [u8; 0]`
"#]];
    let renderer = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn short_error_format1() {
    // tests/ui/short-error-format.rs

    let source = r#"//@ compile-flags: --error-format=short

fn foo(_: u32) {}

fn main() {
    foo("Bonjour".to_owned());
    let x = 0u32;
    x.salut();
}
"#;

    let input = &[
        Group::new()
            .element(Level::ERROR.title("mismatched types").id("E0308"))
            .element(
                Snippet::source(source)
                    .path("$DIR/short-error-format.rs")
                    .fold(true)
                    .annotation(
                        AnnotationKind::Primary
                            .span(80..100)
                            .label("expected `u32`, found `String`"),
                    )
                    .annotation(
                        AnnotationKind::Context
                            .span(76..79)
                            .label("arguments to this function are incorrect"),
                    ),
            ),
        Group::new()
            .element(Level::NOTE.title("function defined here"))
            .element(
                Snippet::source(source)
                    .path("$DIR/short-error-format.rs")
                    .fold(true)
                    .annotation(AnnotationKind::Context.span(48..54).label(""))
                    .annotation(AnnotationKind::Primary.span(44..47)),
            ),
    ];

    let expected = str![[r#"
$DIR/short-error-format.rs:6:9: error[E0308]: mismatched types: expected `u32`, found `String`
"#]];
    let renderer = Renderer::plain()
        .short_message(true)
        .anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn short_error_format2() {
    // tests/ui/short-error-format.rs

    let source = r#"//@ compile-flags: --error-format=short

fn foo(_: u32) {}

fn main() {
    foo("Bonjour".to_owned());
    let x = 0u32;
    x.salut();
}
"#;

    let input = &[Group::new()
        .element(
            Level::ERROR
                .title("no method named `salut` found for type `u32` in the current scope")
                .id("E0599"),
        )
        .element(
            Snippet::source(source)
                .path("$DIR/short-error-format.rs")
                .fold(true)
                .annotation(
                    AnnotationKind::Primary
                        .span(127..132)
                        .label("method not found in `u32`"),
                ),
        )];

    let expected = str![[r#"
$DIR/short-error-format.rs:8:7: error[E0599]: no method named `salut` found for type `u32` in the current scope: method not found in `u32`
"#]];
    let renderer = Renderer::plain()
        .short_message(true)
        .anonymized_line_numbers(true);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn rustdoc_ui_diagnostic_width() {
    // tests/rustdoc-ui/diagnostic-width.rs

    let source_0 = r#"//@ compile-flags: --diagnostic-width=10
#![deny(rustdoc::bare_urls)]

/// This is a long line that contains a http://link.com
pub struct Foo; //~^ ERROR
"#;
    let source_1 = r#"/// This is a long line that contains a http://link.com
"#;

    let input = &[
        Group::new()
            .element(Level::ERROR.title("this URL is not a hyperlink"))
            .element(
                Snippet::source(source_0)
                    .path("$DIR/diagnostic-width.rs")
                    .fold(true)
                    .annotation(AnnotationKind::Primary.span(111..126)),
            )
            .element(
                Level::NOTE.title("bare URLs are not automatically turned into clickable links"),
            ),
        Group::new()
            .element(Level::NOTE.title("the lint level is defined here"))
            .element(
                Snippet::source(source_0)
                    .path("$DIR/diagnostic-width.rs")
                    .fold(true)
                    .annotation(AnnotationKind::Primary.span(49..67)),
            ),
        Group::new()
            .element(Level::HELP.title("use an automatic link instead"))
            .element(
                Snippet::source(source_1)
                    .path("$DIR/diagnostic-width.rs")
                    .line_start(4)
                    .fold(true)
                    .patch(Patch::new(40..40, "<"))
                    .patch(Patch::new(55..55, ">")),
            ),
    ];

    let expected = str![[r#"
error: this URL is not a hyperlink
  --> $DIR/diagnostic-width.rs:4:41
   |
LL | ... a http://link.com
   |       ^^^^^^^^^^^^^^^
   |
   = note: bare URLs are not automatically turned into clickable links
note: the lint level is defined here
  --> $DIR/diagnostic-width.rs:2:9
   |
LL | ...ny(ru...are_urls)]
   |       ^^...^^^^^^^^
help: use an automatic link instead
   |
LL | /// This is a long line that contains a <http://link.com>
   |                                         +               +
"#]];
    let renderer = Renderer::plain()
        .anonymized_line_numbers(true)
        .term_width(10);
    assert_data_eq!(renderer.render(input), expected);
}

#[test]
fn array_into_iter() {
    let source1 = r#"#![allow(unused)]
fn main() {
[1, 2, 3].into_iter().for_each(|n| { *n; });
}
"#;
    let source2 = r#"[1, 2, 3].into_iter().for_each(|n| { *n; });
"#;

    let long_title1 ="this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to `<[T; N] as IntoIterator>::into_iter` in Rust 2021";
    let long_title2 = "for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>";
    let long_title3 = "or use `IntoIterator::into_iter(..)` instead of `.into_iter()` to explicitly iterate by value";

    let input = &[
        Group::new()
            .element(Level::WARNING.title(long_title1))
            .element(
                Snippet::source(source1)
                    .path("lint_example.rs")
                    .fold(true)
                    .annotation(AnnotationKind::Primary.span(40..49)),
            )
            .element(Level::WARNING.title("this changes meaning in Rust 2021"))
            .element(Level::NOTE.title(long_title2))
            .element(Level::NOTE.title("`#[warn(array_into_iter)]` on by default")),
        Group::new()
            .element(
                Level::HELP.title("use `.iter()` instead of `.into_iter()` to avoid ambiguity"),
            )
            .element(
                Snippet::source(source2)
                    .path("lint_example.rs")
                    .line_start(3)
                    .fold(true)
                    .patch(Patch::new(10..19, "iter")),
            ),
        Group::new()
            .element(Level::HELP.title(long_title3))
            .element(
                Snippet::source(source2)
                    .path("lint_example.rs")
                    .line_start(3)
                    .fold(true)
                    .patch(Patch::new(0..0, "IntoIterator::into_iter("))
                    .patch(Patch::new(9..21, ")")),
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
3 - [1, 2, 3].into_iter().for_each(|n| { *n; });
3 + IntoIterator::into_iter([1, 2, 3]).for_each(|n| { *n; });
  |
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected);
}
