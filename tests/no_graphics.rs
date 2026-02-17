use annotate_snippets::{AnnotationKind, Level, Padding, Patch, Renderer, Snippet};

use annotate_snippets::renderer::DecorStyle;
use snapbox::{IntoData, assert_data_eq, str};

#[test]
fn missing_fields_in_builder() {
    let source = r#"use bitbybit::bitfield;

#[bitfield(u32, default = 0, forbid_overlaps)]
struct Test {
    #[bits(8..=15, rw)]
    foo: u8,
    #[bits(0..=7, rw)]
    bar: u8,
}

fn main() {
    Test::builder().with_foo(1).build();
    Test::builder().with_bar(1).build();
    Test::builder().with_bar(1).with_foo(1).build();
    Test::builder().with_bar(1).with_foo(1).with_bar(2).build();
}
"#;
    let title =
        "no method named `build` found for struct `PartialTest<true, false>` in the current scope";
    let path = "tests/no_compile/missing_fields_in_builder.rs";

    let report = &[Level::ERROR
        .primary_title(title)
        .id("E0599")
        .element(
            Snippet::source(source)
                .path(path)
                .annotation(
                    AnnotationKind::Primary
                        .span(206..211)
                        .label("method not found in `PartialTest<true, false>`"),
                )
                .annotation(
                    AnnotationKind::Context
                        .span(25..71)
                        .label("method `build` not found for this struct"),
                ),
        )
        .element(Level::NOTE.message("the method was found for\n- `PartialTest<true, true>`"))];

    let expected_ascii = str![[r#"
error[E0599]: no method named `build` found for struct `PartialTest<true, false>` in the current scope
  --> tests/no_compile/missing_fields_in_builder.rs:12:33
   |
 3 | #[bitfield(u32, default = 0, forbid_overlaps)]
   | ---------------------------------------------- method `build` not found for this struct
...
12 |     Test::builder().with_foo(1).build();
   |                                 ^^^^^ method not found in `PartialTest<true, false>`
   |
   = note: the method was found for
           - `PartialTest<true, true>`
"#]];
    let renderer_ascii = Renderer::plain().decor_style(DecorStyle::Ascii);
    assert_data_eq!(renderer_ascii.render(report), expected_ascii);

    let expected_no_graphics = str![[r#"
error[E0599]: no method named `build` found for struct `PartialTest<true, false>` in the current scope
  --> tests/no_compile/missing_fields_in_builder.rs:12:33
   |
 3 | #[bitfield(u32, default = 0, forbid_overlaps)]
   | ---------------------------------------------- method `build` not found for this struct
...
12 |     Test::builder().with_foo(1).build();
   |                                 ^^^^^ method not found in `PartialTest<true, false>`
   |
   = note: the method was found for
           - `PartialTest<true, true>`
"#]];
    let renderer_no_graphics = renderer_ascii;
    assert_data_eq!(renderer_no_graphics.render(report), expected_no_graphics);
}

#[test]
fn missing_fields_in_builder2() {
    let source = r#"use bitbybit::bitfield;

#[bitfield(u32, default = 0, forbid_overlaps)]
struct Test {
    #[bits(8..=15, rw)]
    foo: u8,
    #[bits(0..=7, rw)]
    bar: u8,
}

fn main() {
    Test::builder().with_foo(1).build();
    Test::builder().with_bar(1).build();
    Test::builder().with_bar(1).with_foo(1).build();
    Test::builder().with_bar(1).with_foo(1).with_bar(2).build();
}
"#;
    let title = "no method named `with_bar` found for struct `PartialTest<true, true>` in the current scope";
    let path = "tests/no_compile/missing_fields_in_builder.rs";

    let report = &[
        Level::ERROR
            .primary_title(title)
            .id("E0599")
            .element(
                Snippet::source(source)
                    .path(path)
                    .annotation(
                        AnnotationKind::Primary
                            .span(353..361)
                            .label("method not found in `PartialTest<true, true>`"),
                    )
                    .annotation(
                        AnnotationKind::Context
                            .span(25..71)
                            .label("method `with_bar` not found for this struct"),
                    ),
            )
            .element(
                Level::NOTE
                    .message("the method was found for\n- `PartialTest<foo_bitfield, false>`"),
            ),
        Level::HELP
            .primary_title("one of the expressions' fields has a method of the same name")
            .element(
                Snippet::source(source)
                    .path(path)
                    .patch(Patch::new(353..353, "value.")),
            ),
    ];

    let expected_ascii = str![[r#"
error[E0599]: no method named `with_bar` found for struct `PartialTest<true, true>` in the current scope
  --> tests/no_compile/missing_fields_in_builder.rs:15:45
   |
 3 | #[bitfield(u32, default = 0, forbid_overlaps)]
   | ---------------------------------------------- method `with_bar` not found for this struct
...
15 |     Test::builder().with_bar(1).with_foo(1).with_bar(2).build();
   |                                             ^^^^^^^^ method not found in `PartialTest<true, true>`
   |
   = note: the method was found for
           - `PartialTest<foo_bitfield, false>`
help: one of the expressions' fields has a method of the same name
   |
15 |     Test::builder().with_bar(1).with_foo(1).value.with_bar(2).build();
   |                                             ++++++
"#]];
    let renderer_ascii = Renderer::plain().decor_style(DecorStyle::Ascii);
    assert_data_eq!(renderer_ascii.render(report), expected_ascii);

    let expected_no_graphics = str![[r#"
error[E0599]: no method named `with_bar` found for struct `PartialTest<true, true>` in the current scope
  --> tests/no_compile/missing_fields_in_builder.rs:15:45
   |
 3 | #[bitfield(u32, default = 0, forbid_overlaps)]
   | ---------------------------------------------- method `with_bar` not found for this struct
...
15 |     Test::builder().with_bar(1).with_foo(1).with_bar(2).build();
   |                                             ^^^^^^^^ method not found in `PartialTest<true, true>`
   |
   = note: the method was found for
           - `PartialTest<foo_bitfield, false>`
help: one of the expressions' fields has a method of the same name
   |
15 |     Test::builder().with_bar(1).with_foo(1).value.with_bar(2).build();
   |                                             ++++++
"#]];
    let renderer_no_graphics = renderer_ascii;
    assert_data_eq!(renderer_no_graphics.render(report), expected_no_graphics);
}

#[test]
fn missing_type() {
    let source = r#"//@ edition: 2015
//@ compile-flags: --error-format human

pub fn main() {
    let x: Iter;
}

//~? RAW cannot find type `Iter` in this scope
"#;
    let path = "$DIR/missing-type.rs";

    let report = &[
        Level::ERROR
            .primary_title("cannot find type `Iter` in this scope")
            .id("E0425")
            .element(
                Snippet::source(source).path(path).annotation(
                    AnnotationKind::Primary
                        .span(86..90)
                        .label("not found in this scope"),
                ),
            ),
        Level::HELP
            .secondary_title("consider importing one of these structs")
            .element(Snippet::source(source).path(path).patch(Patch::new(
                59..59,
                "use std::collections::binary_heap::Iter;\n\n",
            )))
            .element(Snippet::source(source).path(path).patch(Patch::new(
                59..59,
                "use std::collections::btree_map::Iter;\n\n",
            )))
            .element(Snippet::source(source).path(path).patch(Patch::new(
                59..59,
                "use std::collections::btree_set::Iter;\n\n",
            )))
            .element(Snippet::source(source).path(path).patch(Patch::new(
                59..59,
                "use std::collections::hash_map::Iter;\n\n",
            )))
            .element(Level::NOTE.no_name().message("and 9 other candidates")),
    ];

    let expected_ascii = str![[r#"
error[E0425]: cannot find type `Iter` in this scope
 --> $DIR/missing-type.rs:5:12
  |
5 |     let x: Iter;
  |            ^^^^ not found in this scope
  |
help: consider importing one of these structs
  |
4 + use std::collections::binary_heap::Iter;
  |
4 + use std::collections::btree_map::Iter;
  |
4 + use std::collections::btree_set::Iter;
  |
4 + use std::collections::hash_map::Iter;
  |
  = and 9 other candidates
"#]];
    let renderer_ascii = Renderer::plain();
    assert_data_eq!(renderer_ascii.render(report), expected_ascii);

    let expected_no_graphics = str![[r#"
error[E0425]: cannot find type `Iter` in this scope
 --> $DIR/missing-type.rs:5:12
  |
5 |     let x: Iter;
  |            ^^^^ not found in this scope
  |
help: consider importing one of these structs
  |
4 + use std::collections::binary_heap::Iter;
  |
4 + use std::collections::btree_map::Iter;
  |
4 + use std::collections::btree_set::Iter;
  |
4 + use std::collections::hash_map::Iter;
  |
  = and 9 other candidates
"#]];
    let renderer_no_graphics = renderer_ascii;
    assert_data_eq!(renderer_no_graphics.render(report), expected_no_graphics);
}

#[test]
fn multiple_files() {
    let source_og = r#"//@ aux-build:other_file.rs
//@ compile-flags: --error-format human

extern crate other_file;

fn main() {
    other_file::WithPrivateMethod.private_method();
}"#;

    let source_og1 = r#"pub struct WithPrivateMethod;

impl WithPrivateMethod {
    /// Private to get an error involving two files
    fn private_method(&self) {}
}
"#;

    let report = &[Level::ERROR
        .primary_title("method `private_method` is private")
        .id("E0624")
        .element(
            Snippet::source(source_og)
                .path("$DIR/multiple-files.rs")
                .annotation(
                    AnnotationKind::Primary
                        .span(141..155)
                        .label("private method"),
                ),
        )
        .element(
            Snippet::source(source_og1)
                .path("$DIR/auxiliary/other_file.rs")
                .annotation(
                    AnnotationKind::Context
                        .span(112..136)
                        .label("private method defined here"),
                ),
        )];

    let expected_ascii = str![[r#"
error[E0624]: method `private_method` is private
  --> $DIR/multiple-files.rs:7:35
   |
LL |     other_file::WithPrivateMethod.private_method();
   |                                   ^^^^^^^^^^^^^^ private method
   |
  ::: $DIR/auxiliary/other_file.rs:5:5
   |
LL |     fn private_method(&self) {}
   |     ------------------------ private method defined here
"#]];
    let renderer_ascii = Renderer::plain().anonymized_snippet_line_numbers(true);
    assert_data_eq!(renderer_ascii.render(report), expected_ascii);

    let expected_no_graphics = str![[r#"
error[E0624]: method `private_method` is private
  --> $DIR/multiple-files.rs:7:35
   |
LL |     other_file::WithPrivateMethod.private_method();
   |                                   ^^^^^^^^^^^^^^ private method
   |
  ::: $DIR/auxiliary/other_file.rs:5:5
   |
LL |     fn private_method(&self) {}
   |     ------------------------ private method defined here
"#]];
    let renderer_no_graphics = renderer_ascii;
    assert_data_eq!(renderer_no_graphics.render(report), expected_no_graphics);
}

// The second unlabeled primary annotation is underlined in graphical output,
// but its location is missing from no-graphics output.
#[test]
fn multispan() {
    let source = r#"//@ proc-macro: multispan.rs
//@ compile-flags: --error-format human

#![feature(proc_macro_hygiene)]

extern crate multispan;

use multispan::hello;

fn main() {
    // This one emits no error.
    hello!();

    // Exactly one 'hi'.
    hello!(hi);

    // Now two, back to back.
    hello!(hi hi);

    // Now three, back to back.
    hello!(hi hi hi);

    // Now several, with spacing.
    hello!(hi hey hi yo hi beep beep hi hi);
    hello!(hi there, hi how are you? hi... hi.);
    hello!(whoah. hi di hi di ho);
    hello!(hi good hi and good bye);
}

//~? RAW hello to you, too!"#;
    let message = "this error originates in the macro `hello` (in Nightly builds, run with -Z macro-backtrace for more info)";

    let report = &[
        Level::ERROR.primary_title("hello to you, too!").element(
            Snippet::source(source)
                .path("$DIR/multispan.rs")
                .annotation(AnnotationKind::Primary.span(286..299)),
        ),
        Level::NOTE
            .secondary_title("found these 'hi's")
            .element(
                Snippet::source(source)
                    .path("$DIR/multispan.rs")
                    .annotation(AnnotationKind::Primary.span(293..295))
                    .annotation(AnnotationKind::Primary.span(296..298)),
            )
            .element(Level::NOTE.message(message)),
    ];

    let expected_ascii = str![[r#"
error: hello to you, too!
  --> $DIR/multispan.rs:18:5
   |
LL |     hello!(hi hi);
   |     ^^^^^^^^^^^^^
   |
note: found these 'hi's
  --> $DIR/multispan.rs:18:12
   |
LL |     hello!(hi hi);
   |            ^^ ^^
   = note: this error originates in the macro `hello` (in Nightly builds, run with -Z macro-backtrace for more info)
"#]];
    let renderer_ascii = Renderer::plain().anonymized_snippet_line_numbers(true);
    assert_data_eq!(renderer_ascii.render(report), expected_ascii);

    let expected_unicode = str![[r#"
error: hello to you, too!
   ╭▸ $DIR/multispan.rs:18:5
   │
LL │     hello!(hi hi);
   │     ━━━━━━━━━━━━━
   ╰╴
note: found these 'hi's
   ╭▸ $DIR/multispan.rs:18:12
   │
LL │     hello!(hi hi);
   │            ━━ ━━
   ╰ note: this error originates in the macro `hello` (in Nightly builds, run with -Z macro-backtrace for more info)
"#]];
    let renderer_unicode = renderer_ascii.clone().decor_style(DecorStyle::Unicode);
    assert_data_eq!(renderer_unicode.render(report), expected_unicode);

    let expected_no_graphics = str![[r#"
error: hello to you, too!
  --> $DIR/multispan.rs:18:5
   |
LL |     hello!(hi hi);
   |     ^^^^^^^^^^^^^
   |
note: found these 'hi's
  --> $DIR/multispan.rs:18:12
   |
LL |     hello!(hi hi);
   |            ^^ ^^
   = note: this error originates in the macro `hello` (in Nightly builds, run with -Z macro-backtrace for more info)
"#]];
    let renderer_no_graphics = renderer_ascii;
    assert_data_eq!(renderer_no_graphics.render(report), expected_no_graphics);
}

#[test]
fn foreign_suggestion() {
    let source1 = r#"
fn main() {
    hello!();
}
"#;
    let source2 = r#"
macro_rules! hello {
    () => {{ foo() }}
}
"#;
    let report = &[
        Level::ERROR.primary_title("foo").element(
            Snippet::source(source1)
                .path("$DIR/foo.rs")
                .annotation(AnnotationKind::Primary.span(17..25)),
        ),
        Level::HELP
            .secondary_title("consider removing this")
            .element(
                Snippet::source(source2)
                    .path("$DIR/macro.rs")
                    .patch(Patch::new(35..40, "")),
            ),
    ];

    let expected_ascii = str![[r#"
error: foo
  --> $DIR/foo.rs:3:5
   |
LL |     hello!();
   |     ^^^^^^^^
   |
help: consider removing this
  --> $DIR/macro.rs:3:14
   |
LL -     () => {{ foo() }}
LL +     () => {{  }}
   |
"#]];
    let renderer_ascii = Renderer::plain().anonymized_snippet_line_numbers(true);
    assert_data_eq!(renderer_ascii.render(report), expected_ascii);

    let expected_no_graphics = str![[r#"
error: foo
  --> $DIR/foo.rs:3:5
   |
LL |     hello!();
   |     ^^^^^^^^
   |
help: consider removing this
  --> $DIR/macro.rs:3:14
   |
LL -     () => {{ foo() }}
LL +     () => {{  }}
   |
"#]];
    let renderer_no_graphics = renderer_ascii;
    assert_data_eq!(renderer_no_graphics.render(report), expected_no_graphics);
}

#[test]
fn suggestion_renders_all_patches() {
    let source = "x = value + 1;\n";
    let report = &[
        Level::ERROR.primary_title("mismatched types").element(
            Snippet::source(source)
                .path("file.rs")
                .annotation(AnnotationKind::Primary.span(4..9).label("expected `u8`")),
        ),
        Level::HELP
            .secondary_title("convert both operands")
            .element(
                Snippet::source(source)
                    .path("file.rs")
                    .patch(Patch::new(4..9, "u8::from(value)"))
                    .patch(Patch::new(12..13, "1u8")),
            ),
    ];

    let expected_ascii = str![[r#"
error: mismatched types
 --> file.rs:1:5
  |
1 | x = value + 1;
  |     ^^^^^ expected `u8`
  |
help: convert both operands
  |
1 - x = value + 1;
1 + x = u8::from(value) + 1u8;
  |
"#]];
    let renderer_ascii = Renderer::plain();
    assert_data_eq!(renderer_ascii.render(report), expected_ascii);

    let expected_no_graphics = str![[r#"
error: mismatched types
 --> file.rs:1:5
  |
1 | x = value + 1;
  |     ^^^^^ expected `u8`
  |
help: convert both operands
  |
1 - x = value + 1;
1 + x = u8::from(value) + 1u8;
  |
"#]];
    let renderer_no_graphics = renderer_ascii;
    assert_data_eq!(renderer_no_graphics.render(report), expected_no_graphics);
}

#[test]
fn suggestion_location_tracks_trimmed_patch() {
    let source = "call(arg);\n";
    let report = &[
        Level::ERROR.primary_title("mismatched types").element(
            Snippet::source(source)
                .path("file.rs")
                .annotation(AnnotationKind::Primary.span(5..8).label("expected `u8`")),
        ),
        Level::HELP.secondary_title("convert it").element(
            Snippet::source(source)
                .path("file.rs")
                .patch(Patch::new(5..8, "arg.into()")),
        ),
    ];

    let expected_ascii = str![[r#"
error: mismatched types
 --> file.rs:1:6
  |
1 | call(arg);
  |      ^^^ expected `u8`
  |
help: convert it
  |
1 | call(arg.into());
  |         +++++++
"#]];
    let renderer_ascii = Renderer::plain();
    assert_data_eq!(renderer_ascii.render(report), expected_ascii);

    let expected_no_graphics = str![[r#"
error: mismatched types
 --> file.rs:1:6
  |
1 | call(arg);
  |      ^^^ expected `u8`
  |
help: convert it
  |
1 | call(arg.into());
  |         +++++++
"#]];
    let renderer_no_graphics = renderer_ascii;
    assert_data_eq!(renderer_no_graphics.render(report), expected_no_graphics);
}

#[test]
fn suggestion_separated_from_following_note() {
    let source = "use std::slice;\n";
    let report = &[Level::HELP
        .primary_title("consider importing this struct")
        .element(
            Snippet::source(source)
                .path("file.rs")
                .patch(Patch::new(8..8, "slice::")),
        )
        .element(Level::NOTE.message("a note after the suggestion"))];

    let expected_ascii = str![[r#"
help: consider importing this struct
 --> file.rs:1:9
  |
1 | use std:slice:::slice;
  |         +++++++
  = note: a note after the suggestion
"#]];
    let renderer_ascii = Renderer::plain();
    assert_data_eq!(renderer_ascii.render(report), expected_ascii);

    let expected_no_graphics = str![[r#"
help: consider importing this struct
 --> file.rs:1:9
  |
1 | use std:slice:::slice;
  |         +++++++
  = note: a note after the suggestion
"#]];
    let renderer_no_graphics = renderer_ascii;
    assert_data_eq!(renderer_no_graphics.render(report), expected_no_graphics);
}

#[test]
fn padding_between_suggestions() {
    let source = "fn f() {}\n";
    let report = &[Level::HELP
        .primary_title("rename one of these")
        .element(
            Snippet::source(source)
                .path("file.rs")
                .patch(Patch::new(3..4, "a")),
        )
        .element(Padding)
        .element(
            Snippet::source(source)
                .path("file.rs")
                .patch(Patch::new(3..4, "b")),
        )];

    let expected_ascii = str![[r#"
help: rename one of these
 --> file.rs:1:4
  |
1 - fn f() {}
1 + fn a() {}
  |
  |
 --> file.rs:1:4
  |
1 - fn f() {}
1 + fn b() {}
  |
"#]];
    let renderer_ascii = Renderer::plain();
    assert_data_eq!(renderer_ascii.render(report), expected_ascii);

    let expected_no_graphics = str![[r#"
help: rename one of these
 --> file.rs:1:4
  |
1 - fn f() {}
1 + fn a() {}
  |
  |
 --> file.rs:1:4
  |
1 - fn f() {}
1 + fn b() {}
  |
"#]];
    let renderer_no_graphics = renderer_ascii;
    assert_data_eq!(renderer_no_graphics.render(report), expected_no_graphics);
}

#[test]
fn multibyte_line_column_heuristic() {
    let source = "\u{a0}abc\n";
    let report = &[Level::ERROR.primary_title("mismatched types").element(
        Snippet::source(source)
            .path("file.rs")
            .annotation(AnnotationKind::Primary.span(4..5).label("primary label"))
            .annotation(AnnotationKind::Context.span(3..4).label("context label")),
    )];

    let expected_ascii = str![[r#"
error: mismatched types
 --> file.rs:1:4
  |
1 |  abc
  |   -^ primary label
  |   |
  |   context label
"#]];
    let renderer_ascii = Renderer::plain();
    assert_data_eq!(renderer_ascii.render(report), expected_ascii);

    let expected_no_graphics = str![[r#"
error: mismatched types
 --> file.rs:1:4
  |
1 |  abc
  |   -^ primary label
  |   |
  |   context label
"#]];
    let renderer_no_graphics = renderer_ascii;
    assert_data_eq!(renderer_no_graphics.render(report), expected_no_graphics);
}

#[test]
fn suggestion_path_without_primary_path() {
    let source = "fn main() { let x: Iter; }\n";
    let report = &[Level::HELP
        .primary_title("consider importing this struct")
        .element(
            Snippet::source(source)
                .path("b.rs")
                .patch(Patch::new(0..0, "use std::slice::Iter;\n")),
        )];

    let expected_ascii = str![[r#"
help: consider importing this struct
 --> b.rs:1:1
  |
1 + use std::slice::Iter;
  |
"#]];
    let renderer_ascii = Renderer::plain();
    assert_data_eq!(renderer_ascii.render(report), expected_ascii);

    let expected_no_graphics = str![[r#"
help: consider importing this struct
 --> b.rs:1:1
  |
1 + use std::slice::Iter;
  |
"#]];
    let renderer_no_graphics = renderer_ascii;
    assert_data_eq!(renderer_no_graphics.render(report), expected_no_graphics);
}

#[test]
fn control_characters_unsanitized() {
    let source = "let x = 1;\n";
    let report = &[Level::ERROR.primary_title("tab\there").element(
        Snippet::source(source)
            .path("file.rs")
            .annotation(AnnotationKind::Primary.span(4..5).label("label\twith tab")),
    )];

    let expected_ascii = str![[r#"
error: tab    here
 --> file.rs:1:5
  |
1 | let x = 1;
  |     ^ label	with tab
"#]];
    let renderer_ascii = Renderer::plain();
    assert_data_eq!(renderer_ascii.render(report), expected_ascii);

    let expected_no_graphics = str![[r#"
error: tab    here
 --> file.rs:1:5
  |
1 | let x = 1;
  |     ^ label	with tab
"#]];
    let renderer_no_graphics = renderer_ascii;
    assert_data_eq!(renderer_no_graphics.render(report), expected_no_graphics);
}

#[test]
fn anonymized_origin_line_numbers_with_no_graphics() {
    let source = "fn main() {\n    call(arg);\n}\n";
    let report = &[Level::ERROR.primary_title("mismatched types").element(
        Snippet::source(source)
            .path("$DIR/file.rs")
            .annotation(AnnotationKind::Primary.span(21..24).label("expected `u8`")),
    )];

    let expected_ascii = str![[r#"
error: mismatched types
 --> $DIR/file.rs:LL:10
  |
2 |     call(arg);
  |          ^^^ expected `u8`
"#]];
    let renderer_ascii = Renderer::plain().anonymized_origin_line_numbers(true);
    assert_data_eq!(renderer_ascii.render(report), expected_ascii);

    let expected_no_graphics = str![[r#"
error: mismatched types
 --> $DIR/file.rs:LL:10
  |
2 |     call(arg);
  |          ^^^ expected `u8`
"#]];
    let renderer_no_graphics = renderer_ascii;
    assert_data_eq!(renderer_no_graphics.render(report), expected_no_graphics);
}

#[test]
fn alternative_suggestions_preserve_locations() {
    let source = "first\nsecond";
    let report = &[Level::HELP
        .primary_title("change either declaration")
        .element(
            Snippet::source(source)
                .path("file.rs")
                .patch(Patch::new(0..5, "x")),
        )
        .element(
            Snippet::source(source)
                .path("file.rs")
                .patch(Patch::new(6..12, "y")),
        )];

    let expected_ascii = str![[r#"
help: change either declaration
 --> file.rs:1:1
  |
1 - first
1 + x
  |
2 - second
2 + y
  |
"#]];
    let renderer_ascii = Renderer::plain();
    assert_data_eq!(renderer_ascii.render(report), expected_ascii);

    let expected_no_graphics = str![[r#"
help: change either declaration
 --> file.rs:1:1
  |
1 - first
1 + x
  |
2 - second
2 + y
  |
"#]];
    let renderer_no_graphics = renderer_ascii;
    assert_data_eq!(renderer_no_graphics.render(report), expected_no_graphics);
}

#[test]
fn visible_annotation_does_not_determine_location() {
    let source = "struct Context;\nlet x = 1;\n";
    let report = &[Level::NOTE.primary_title("variable defined here").element(
        Snippet::source(source)
            .path("file.rs")
            .annotation(AnnotationKind::Visible.span(0..15))
            .annotation(AnnotationKind::Context.span(20..21)),
    )];

    let expected_ascii = str![[r#"
note: variable defined here
 --> file.rs:2:5
  |
1 | struct Context;
2 | let x = 1;
  |     -
"#]];
    let renderer_ascii = Renderer::plain();
    assert_data_eq!(renderer_ascii.render(report), expected_ascii);

    let expected_no_graphics = str![[r#"
note: variable defined here
 --> file.rs:2:5
  |
1 | struct Context;
2 | let x = 1;
  |     -
"#]];
    let renderer_no_graphics = renderer_ascii;
    assert_data_eq!(renderer_no_graphics.render(report), expected_no_graphics);
}

#[test]
fn multibyte_suggestion_column_heuristic() {
    let source = "\u{2003}xyabc";
    let report = &[Level::HELP.primary_title("replace the suffix").element(
        Snippet::source(source)
            .path("file.rs")
            .patch(Patch::new(5..8, "first\nsecond")),
    )];

    let expected_ascii = str![[r#"
help: replace the suffix
 --> file.rs:1:4
  |
1 ~  xyfirst
2 + second
  |
"#]];
    let renderer_ascii = Renderer::plain();
    assert_data_eq!(renderer_ascii.render(report), expected_ascii);

    let expected_no_graphics = str![[r#"
help: replace the suffix
 --> file.rs:1:4
  |
1 ~  xyfirst
2 + second
  |
"#]];
    let renderer_no_graphics = renderer_ascii;
    assert_data_eq!(renderer_no_graphics.render(report), expected_no_graphics);
}

#[test]
fn patch_replacement_escapes_terminal_controls() {
    let report = &[Level::HELP.primary_title("change the value").element(
        Snippet::source("old")
            .path("file.rs")
            .patch(Patch::new(0..3, "\x1b[2Jvalue")),
    )];

    let expected_ascii =
        str![[r#""help: change the value\n --> file.rs:1:1\n  |\n1 - old\n1 + ␛[2Jvalue\n  |""#]]
            .raw();
    let renderer_ascii = Renderer::plain();
    assert_data_eq!(
        format!("{:?}", renderer_ascii.render(report)),
        expected_ascii
    );

    let expected_no_graphics =
        str![[r#""help: change the value\n --> file.rs:1:1\n  |\n1 - old\n1 + ␛[2Jvalue\n  |""#]]
            .raw();
    let renderer_no_graphics = renderer_ascii;
    assert_data_eq!(
        format!("{:?}", renderer_no_graphics.render(report)),
        expected_no_graphics,
    );
}

#[test]
fn alternative_patch_replacement_normalizes_bidi_controls() {
    let report = &[Level::HELP
        .primary_title("change the value")
        .element(
            Snippet::source("old")
                .path("file.rs")
                .patch(Patch::new(0..3, "\u{202e}value\u{202c}")),
        )
        .element(
            Snippet::source("old")
                .path("file.rs")
                .patch(Patch::new(0..3, "other")),
        )];

    let expected_ascii = str![[r#""help: change the value\n --> file.rs:1:1\n  |\n1 - old\n1 + �value�\n  |\n1 - old\n1 + other\n  |""#]].raw();
    let renderer_ascii = Renderer::plain();
    assert_data_eq!(
        format!("{:?}", renderer_ascii.render(report)),
        expected_ascii
    );

    let expected_no_graphics = str![[r#""help: change the value\n --> file.rs:1:1\n  |\n1 - old\n1 + �value�\n  |\n1 - old\n1 + other\n  |""#]]
    .raw();
    let renderer_no_graphics = renderer_ascii;
    assert_data_eq!(
        format!("{:?}", renderer_no_graphics.render(report)),
        expected_no_graphics,
    );
}
