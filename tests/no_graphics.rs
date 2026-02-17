use annotate_snippets::{AnnotationKind, Level, Patch, Renderer, Snippet};

use annotate_snippets::renderer::DecorStyle;
use snapbox::{assert_data_eq, str};

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
error E0599: no method named `build` found for struct `PartialTest<true, false>` in the current scope
at tests/no_compile/missing_fields_in_builder.rs, on line 12, column 33: method not found in `PartialTest<true, false>`
 on line 3: method `build` not found for this struct
note: the method was found for
      - `PartialTest<true, true>`
"#]];
    let renderer_no_graphics = Renderer::plain().no_graphics(true);
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
error E0599: no method named `with_bar` found for struct `PartialTest<true, true>` in the current scope
at tests/no_compile/missing_fields_in_builder.rs, on line 15, column 45: method not found in `PartialTest<true, true>`
 on line 3: method `with_bar` not found for this struct
note: the method was found for
      - `PartialTest<foo_bitfield, false>`
help: one of the expressions' fields has a method of the same name: at line 15, column 44, add `value.`
"#]];
    let renderer_no_graphics = Renderer::plain().no_graphics(true);
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
error E0425: cannot find type `Iter` in this scope
at $DIR/missing-type.rs, on line 5, column 12: not found in this scope
help: consider importing one of these structs: at line 4, column 1, add one of `use std::collections::binary_heap::Iter;`, `use std::collections::btree_map::Iter;`, `use std::collections::btree_set::Iter;`, `use std::collections::hash_map::Iter;` or 9 other candidates
"#]];
    let renderer_no_graphics = Renderer::plain().no_graphics(true);
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
    let renderer_ascii = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer_ascii.render(report), expected_ascii);

    let expected_no_graphics = str![[r#"
error E0624: method `private_method` is private
at $DIR/multiple-files.rs, on line 7, column 35: private method
at $DIR/auxiliary/other_file.rs, on line 5, column 5: private method defined here
"#]];
    let renderer_no_graphics = Renderer::plain().no_graphics(true);
    assert_data_eq!(renderer_no_graphics.render(report), expected_no_graphics);
}

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
    let renderer_ascii = Renderer::plain().anonymized_line_numbers(true);
    assert_data_eq!(renderer_ascii.render(report), expected_ascii);

    let expected_no_graphics = str![[r#"
error: hello to you, too!
at $DIR/multispan.rs, on line 18, column 5
note: found these 'hi's
at $DIR/multispan.rs, on line 18, column 12
note: this error originates in the macro `hello` (in Nightly builds, run with -Z macro-backtrace for more info)
"#]];
    let renderer_no_graphics = Renderer::plain().no_graphics(true);
    assert_data_eq!(renderer_no_graphics.render(report), expected_no_graphics);
}
