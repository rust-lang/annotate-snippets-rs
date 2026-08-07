use annotate_snippets::renderer::DecorStyle;
use annotate_snippets::{Annotation, AnnotationKind, Group, Level, Patch, Renderer, Snippet};
use snapbox::{assert_data_eq, str};

#[test]
fn example_code() {
    let input = &[
        Level::ERROR
            .primary_title("Invalid override of method `__eq__`")
            .id("invalid-method-override")
            .element(
                Snippet::source("    def __eq__(self, other) -> NotBoolable:")
                    .path("src/mdtest_snippet.py")
                    .line_start(6)
                    .annotation(
                        AnnotationKind::Primary
                            .span(8..42)
                            .label("Definition is incompatible with `object.__eq__`"),
                    ),
            ),
        Group::with_title(Level::INFO.secondary_title(
            "incompatible return types: `NotBoolable` is not assignable to `bool`",
        )),
        Group::with_title(
            Level::HELP.secondary_title("This violates the Liskov Substitution Principle"),
        ),
        Level::HELP
            .secondary_title(
                "It is recommended for `__eq__` to work with arbitrary objects, for example:",
            )
            .element(
                Snippet::<Annotation<'_>>::source(
                    "\
    def __eq__(self, other: object) -> bool:
        if not isinstance(other, A):
            return False
        return <logic to compare two `A` instances>
",
                )
                .line_numbering(false)
                .fold(false),
            ),
    ];

    let expected_ascii = str![[r#"
error[invalid-method-override]: Invalid override of method `__eq__`
 --> src/mdtest_snippet.py:6:9
  |
6 |     def __eq__(self, other) -> NotBoolable:
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Definition is incompatible with `object.__eq__`
  |
info: incompatible return types: `NotBoolable` is not assignable to `bool`
help: This violates the Liskov Substitution Principle
help: It is recommended for `__eq__` to work with arbitrary objects, for example:
  |
  | def __eq__(self, other: object) -> bool:
  |         if not isinstance(other, A):
  |             return False
  |         return <logic to compare two `A` instances>
  |
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected_ascii);

    let expected_unicode = str![[r#"
error[invalid-method-override]: Invalid override of method `__eq__`
  ╭▸ src/mdtest_snippet.py:6:9
  │
6 │     def __eq__(self, other) -> NotBoolable:
  │         ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Definition is incompatible with `object.__eq__`
  ╰╴
info: incompatible return types: `NotBoolable` is not assignable to `bool`
help: This violates the Liskov Substitution Principle
help: It is recommended for `__eq__` to work with arbitrary objects, for example:
  ╭▸ 
  │ def __eq__(self, other: object) -> bool:
  │         if not isinstance(other, A):
  │             return False
  │         return <logic to compare two `A` instances>
  ╰╴
"#]];
    let renderer = renderer.decor_style(DecorStyle::Unicode);
    assert_data_eq!(renderer.render(input), expected_unicode);

    let expected_no_graphics = str![[r#"
error invalid-method-override: Invalid override of method `__eq__`
 at src/mdtest_snippet.py:6:9: Definition is incompatible with `object.__eq__`
info: incompatible return types: `NotBoolable` is not assignable to `bool`
help: This violates the Liskov Substitution Principle
help: It is recommended for `__eq__` to work with arbitrary objects, for example:
 on line 1
"#]];
    let renderer_no_graphics = renderer.no_graphics(true);
    assert_data_eq!(renderer_no_graphics.render(input), expected_no_graphics);
}

#[test]
fn fixable_diagnostic_no_name() {
    let input = &[Level::ERROR
        .no_name()
        .primary_title("`os` imported but unused")
        .id("F401")
        .footnote("[*]")
        .element(
            Snippet::source("import os\n")
                .path("-")
                .annotation(AnnotationKind::Primary.span(7..9)),
        )];
    let suffix = "
Found 1 error.
[*] 1 fixable with the `--fix` option.
";

    let expected_ascii = str![[r#"
F401[*]: `os` imported but unused
 --> -:1:8
  |
1 | import os
  |        ^^

Found 1 error.
[*] 1 fixable with the `--fix` option.

"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(
        format!("{}\n{suffix}", renderer.render(input)),
        expected_ascii
    );

    let expected_unicode = str![[r#"
F401[*]: `os` imported but unused
  ╭▸ -:1:8
  │
1 │ import os
  ╰╴       ━━

Found 1 error.
[*] 1 fixable with the `--fix` option.

"#]];
    let renderer = renderer.decor_style(DecorStyle::Unicode);
    assert_data_eq!(
        format!("{}\n{suffix}", renderer.render(input)),
        expected_unicode
    );

    let expected_no_graphics = str![[r#"
F401  footnote [*]: `os` imported but unused
 at -:1:8
"#]];
    let renderer_no_graphics = renderer.no_graphics(true);
    assert_data_eq!(renderer_no_graphics.render(input), expected_no_graphics);
}

#[test]
fn fixable_diagnostic() {
    let input = &[Level::ERROR
        .primary_title("`os` imported but unused")
        .id("F401")
        .footnote("[*]")
        .element(
            Snippet::source("import os\n")
                .path("fib.py")
                .annotation(AnnotationKind::Primary.span(7..9)),
        )];
    let suffix = "
Found 1 error.
[*] 1 fixable with the `--fix` option.
";

    let expected_ascii = str![[r#"
error[F401][*]: `os` imported but unused
 --> fib.py:1:8
  |
1 | import os
  |        ^^

Found 1 error.
[*] 1 fixable with the `--fix` option.

"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(
        format!("{}\n{suffix}", renderer.render(input)),
        expected_ascii
    );

    let expected_unicode = str![[r#"
error[F401][*]: `os` imported but unused
  ╭▸ fib.py:1:8
  │
1 │ import os
  ╰╴       ━━

Found 1 error.
[*] 1 fixable with the `--fix` option.

"#]];
    let renderer = renderer.decor_style(DecorStyle::Unicode);
    assert_data_eq!(
        format!("{}\n{suffix}", renderer.render(input)),
        expected_unicode
    );

    let expected_no_graphics = str![[r#"
error F401  footnote [*]: `os` imported but unused
 at fib.py:1:8
"#]];
    let renderer_no_graphics = renderer.no_graphics(true);
    assert_data_eq!(renderer_no_graphics.render(input), expected_no_graphics);
}

#[test]
fn snippet_with_cell() {
    let source = "First line\r\nSecond oops line";
    let input = &[Level::ERROR.primary_title("oops").element(
        Snippet::source(source).annotation(AnnotationKind::Primary.span(19..23).label("oops")),
    )];

    let expected_ascii = str![[r#"
error: oops
  |
2 | Second oops line
  |        ^^^^ oops
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected_ascii);

    let expected_unicode = str![[r#"
error: oops
  ╭▸ 
2 │ Second oops line
  ╰╴       ━━━━ oops
"#]];
    let renderer = renderer.decor_style(DecorStyle::Unicode);
    assert_data_eq!(renderer.render(input), expected_unicode);

    let expected_no_graphics = str![[r#"
error: oops
 on line 2, column 8: oops
"#]];
    let renderer_no_graphics = renderer.no_graphics(true);
    assert_data_eq!(renderer_no_graphics.render(input), expected_no_graphics);
}

#[test]
fn snippet_with_path_cell() {
    let source = "First line\r\nSecond oops line";
    let input = &[Level::ERROR.primary_title("oops").element(
        Snippet::source(source)
            .path("foo.ipynb")
            .annotation(AnnotationKind::Primary.span(19..23).label("oops")),
    )];

    let expected_ascii = str![[r#"
error: oops
 --> foo.ipynb:2:8
  |
2 | Second oops line
  |        ^^^^ oops
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected_ascii);

    let expected_unicode = str![[r#"
error: oops
  ╭▸ foo.ipynb:2:8
  │
2 │ Second oops line
  ╰╴       ━━━━ oops
"#]];
    let renderer = renderer.decor_style(DecorStyle::Unicode);
    assert_data_eq!(renderer.render(input), expected_unicode);

    let expected_no_graphics = str![[r#"
error: oops
 at foo.ipynb:2:8: oops
"#]];
    let renderer_no_graphics = renderer.no_graphics(true);
    assert_data_eq!(renderer_no_graphics.render(input), expected_no_graphics);
}

#[test]
fn patch_with_primary_cell() {
    let source = "First line\r\nSecond oops line";
    let input = &[
        Level::ERROR.primary_title("oops").element(
            Snippet::source(source).annotation(AnnotationKind::Primary.span(19..23).label("oops")),
        ),
        Level::HELP
            .secondary_title("remove the entry")
            .element(Snippet::source(source).patch(Patch::new(19..24, "")))
            .element(Snippet::source(source).patch(Patch::new(23..28, ""))),
    ];

    let expected_ascii = str![[r#"
error: oops
  |
2 | Second oops line
  |        ^^^^ oops
  |
help: remove the entry
  |
2 - Second oops line
2 + Second line
  |
2 - Second oops line
2 + Second oops
  |
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected_ascii);

    let expected_unicode = str![[r#"
error: oops
  ╭▸ 
2 │ Second oops line
  │        ━━━━ oops
  ╰╴
help: remove the entry
  ╭╴
2 - Second oops line
2 + Second line
  ├╴
2 - Second oops line
2 + Second oops
  ╰╴
"#]];
    let renderer = renderer.decor_style(DecorStyle::Unicode);
    assert_data_eq!(renderer.render(input), expected_unicode);

    let expected_no_graphics = str![[r#"
error: oops
 on line 2, column 8: oops
help: remove the entry
 option 1
  on line 2, column 8
 option 2
  on line 2, column 12
"#]];
    let renderer_no_graphics = renderer.no_graphics(true);
    assert_data_eq!(renderer_no_graphics.render(input), expected_no_graphics);
}

#[test]
fn patch_with_primary_path_cell() {
    let source = "First line\r\nSecond oops line";
    let input = &[
        Level::ERROR.primary_title("oops").element(
            Snippet::source(source)
                .path("foo.ipynb")
                .annotation(AnnotationKind::Primary.span(19..23).label("oops")),
        ),
        Level::HELP
            .secondary_title("remove the entry")
            .element(
                Snippet::source(source)
                    .path("foo.ipynb")
                    .patch(Patch::new(19..24, "")),
            )
            .element(
                Snippet::source(source)
                    .path("foo.ipynb")
                    .patch(Patch::new(23..28, "")),
            ),
    ];

    let expected_ascii = str![[r#"
error: oops
 --> foo.ipynb:2:8
  |
2 | Second oops line
  |        ^^^^ oops
  |
help: remove the entry
  |
2 - Second oops line
2 + Second line
  |
2 - Second oops line
2 + Second oops
  |
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected_ascii);

    let expected_unicode = str![[r#"
error: oops
  ╭▸ foo.ipynb:2:8
  │
2 │ Second oops line
  │        ━━━━ oops
  ╰╴
help: remove the entry
  ╭╴
2 - Second oops line
2 + Second line
  ├╴
2 - Second oops line
2 + Second oops
  ╰╴
"#]];
    let renderer = renderer.decor_style(DecorStyle::Unicode);
    assert_data_eq!(renderer.render(input), expected_unicode);

    let expected_no_graphics = str![[r#"
error: oops
 at foo.ipynb:2:8: oops
help: remove the entry
 option 1
  on line 2, column 8
 option 2
  on line 2, column 12
"#]];
    let renderer_no_graphics = renderer.no_graphics(true);
    assert_data_eq!(renderer_no_graphics.render(input), expected_no_graphics);
}

#[test]
fn patch_with_primary_path_incrementing_cell() {
    let source = "First line\r\nSecond oops line";
    let input = &[
        Level::ERROR.primary_title("oops").element(
            Snippet::source(source)
                .path("foo.ipynb")
                .annotation(AnnotationKind::Primary.span(19..23).label("oops")),
        ),
        Level::HELP
            .secondary_title("remove the entry")
            .element(
                Snippet::source(source)
                    .path("foo.ipynb")
                    .patch(Patch::new(19..24, "")),
            )
            .element(
                Snippet::source(source)
                    .path("foo.ipynb")
                    .patch(Patch::new(23..28, "")),
            ),
    ];

    let expected_ascii = str![[r#"
error: oops
 --> foo.ipynb:2:8
  |
2 | Second oops line
  |        ^^^^ oops
  |
help: remove the entry
  |
2 - Second oops line
2 + Second line
  |
2 - Second oops line
2 + Second oops
  |
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected_ascii);

    let expected_unicode = str![[r#"
error: oops
  ╭▸ foo.ipynb:2:8
  │
2 │ Second oops line
  │        ━━━━ oops
  ╰╴
help: remove the entry
  ╭╴
2 - Second oops line
2 + Second line
  ├╴
2 - Second oops line
2 + Second oops
  ╰╴
"#]];
    let renderer = renderer.decor_style(DecorStyle::Unicode);
    assert_data_eq!(renderer.render(input), expected_unicode);

    let expected_no_graphics = str![[r#"
error: oops
 at foo.ipynb:2:8: oops
help: remove the entry
 option 1
  on line 2, column 8
 option 2
  on line 2, column 12
"#]];
    let renderer_no_graphics = renderer.no_graphics(true);
    assert_data_eq!(renderer_no_graphics.render(input), expected_no_graphics);
}

#[test]
fn patch_with_other_cell() {
    let source = "First line\r\nSecond oops line";
    let input = &[
        Level::ERROR.primary_title("oops").element(
            Snippet::source(source).annotation(AnnotationKind::Primary.span(19..23).label("oops")),
        ),
        Level::HELP
            .secondary_title("remove the entry")
            .element(Snippet::source(source).patch(Patch::new(19..24, "")))
            .element(Snippet::source(source).patch(Patch::new(23..28, ""))),
    ];

    let expected_ascii = str![[r#"
error: oops
  |
2 | Second oops line
  |        ^^^^ oops
  |
help: remove the entry
  |
2 - Second oops line
2 + Second line
  |
2 - Second oops line
2 + Second oops
  |
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected_ascii);

    let expected_unicode = str![[r#"
error: oops
  ╭▸ 
2 │ Second oops line
  │        ━━━━ oops
  ╰╴
help: remove the entry
  ╭╴
2 - Second oops line
2 + Second line
  ├╴
2 - Second oops line
2 + Second oops
  ╰╴
"#]];
    let renderer = renderer.decor_style(DecorStyle::Unicode);
    assert_data_eq!(renderer.render(input), expected_unicode);

    let expected_no_graphics = str![[r#"
error: oops
 on line 2, column 8: oops
help: remove the entry
 option 1
  on line 2, column 8
 option 2
  on line 2, column 12
"#]];
    let renderer_no_graphics = renderer.no_graphics(true);
    assert_data_eq!(renderer_no_graphics.render(input), expected_no_graphics);
}

#[test]
fn patch_with_other_path_cell() {
    let source = "First line\r\nSecond oops line";
    let input = &[
        Level::ERROR.primary_title("oops").element(
            Snippet::source(source)
                .path("foo.ipynb")
                .annotation(AnnotationKind::Primary.span(19..23).label("oops")),
        ),
        Level::HELP
            .secondary_title("remove the entry")
            .element(
                Snippet::source(source)
                    .path("bar.ipynb")
                    .patch(Patch::new(19..24, "")),
            )
            .element(
                Snippet::source(source)
                    .path("bar.ipynb")
                    .patch(Patch::new(23..28, "")),
            ),
    ];

    let expected_ascii = str![[r#"
error: oops
 --> foo.ipynb:2:8
  |
2 | Second oops line
  |        ^^^^ oops
  |
help: remove the entry
 --> bar.ipynb:2:8
  |
2 - Second oops line
2 + Second line
  |
2 - Second oops line
2 + Second oops
  |
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected_ascii);

    let expected_unicode = str![[r#"
error: oops
  ╭▸ foo.ipynb:2:8
  │
2 │ Second oops line
  │        ━━━━ oops
  ╰╴
help: remove the entry
  ╭▸ bar.ipynb:2:8
  │
2 - Second oops line
2 + Second line
  ├╴
2 - Second oops line
2 + Second oops
  ╰╴
"#]];
    let renderer = renderer.decor_style(DecorStyle::Unicode);
    assert_data_eq!(renderer.render(input), expected_unicode);

    let expected_no_graphics = str![[r#"
error: oops
 at foo.ipynb:2:8: oops
help: remove the entry
 option 1
  at bar.ipynb:2:8
 option 2
  on line 2, column 12
"#]];
    let renderer_no_graphics = renderer.no_graphics(true);
    assert_data_eq!(renderer_no_graphics.render(input), expected_no_graphics);
}

#[test]
fn patch_with_other_path_incrementing_cell() {
    let source = "First line\r\nSecond oops line";
    let input = &[
        Level::ERROR.primary_title("oops").element(
            Snippet::source(source)
                .path("foo.ipynb")
                .annotation(AnnotationKind::Primary.span(19..23).label("oops")),
        ),
        Level::HELP
            .secondary_title("remove the entry")
            .element(
                Snippet::source(source)
                    .path("bar.ipynb")
                    .patch(Patch::new(19..24, "")),
            )
            .element(
                Snippet::source(source)
                    .path("bar.ipynb")
                    .patch(Patch::new(23..28, "")),
            ),
    ];

    let expected_ascii = str![[r#"
error: oops
 --> foo.ipynb:2:8
  |
2 | Second oops line
  |        ^^^^ oops
  |
help: remove the entry
 --> bar.ipynb:2:8
  |
2 - Second oops line
2 + Second line
  |
2 - Second oops line
2 + Second oops
  |
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected_ascii);

    let expected_unicode = str![[r#"
error: oops
  ╭▸ foo.ipynb:2:8
  │
2 │ Second oops line
  │        ━━━━ oops
  ╰╴
help: remove the entry
  ╭▸ bar.ipynb:2:8
  │
2 - Second oops line
2 + Second line
  ├╴
2 - Second oops line
2 + Second oops
  ╰╴
"#]];
    let renderer = renderer.decor_style(DecorStyle::Unicode);
    assert_data_eq!(renderer.render(input), expected_unicode);

    let expected_no_graphics = str![[r#"
error: oops
 at foo.ipynb:2:8: oops
help: remove the entry
 option 1
  at bar.ipynb:2:8
 option 2
  on line 2, column 12
"#]];
    let renderer_no_graphics = renderer.no_graphics(true);
    assert_data_eq!(renderer_no_graphics.render(input), expected_no_graphics);
}
