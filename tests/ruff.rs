use annotate_snippets::{Annotation, AnnotationKind, Group, Level, Renderer, Snippet};

use annotate_snippets::renderer::DecorStyle;
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
        if not isinstance(other, A):
            return False
        return <logic to compare two `A` instances>
",
                )
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
1 | def __eq__(self, other: object) -> bool:
2 |         if not isinstance(other, A):
3 |         if not isinstance(other, A):
4 |             return False
5 |         return <logic to compare two `A` instances>
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
1 │ def __eq__(self, other: object) -> bool:
2 │         if not isinstance(other, A):
3 │         if not isinstance(other, A):
4 │             return False
5 │         return <logic to compare two `A` instances>
  ╰╴
"#]];
    let renderer = renderer.decor_style(DecorStyle::Unicode);
    assert_data_eq!(renderer.render(input), expected_unicode);
}
