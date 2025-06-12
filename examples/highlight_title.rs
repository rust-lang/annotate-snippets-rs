use annotate_snippets::{AnnotationKind, Group, Level, Renderer, Snippet};
use anstyle::Effects;

fn main() {
    let source = r#"// Make sure "highlighted" code is colored purple

//@ compile-flags: --error-format=human --color=always
//@ edition:2018

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
}"#;

    let magenta = annotate_snippets::renderer::AnsiColor::Magenta
        .on_default()
        .effects(Effects::BOLD);
    let title = format!(
        "expected fn pointer `{}for<'a>{} fn(Box<{}(dyn Any + Send + 'a){}>) -> Pin<_>`
      found fn item `fn(Box<{}(dyn Any + Send + 'static){}>) -> Pin<_> {}{{wrapped_fn}}{}`",
        magenta.render(),
        magenta.render_reset(),
        magenta.render(),
        magenta.render_reset(),
        magenta.render(),
        magenta.render_reset(),
        magenta.render(),
        magenta.render_reset()
    );

    let message = &[
        Group::new()
            .element(Level::ERROR.title("mismatched types").id("E0308"))
            .element(
                Snippet::source(source)
                    .fold(true)
                    .path("$DIR/highlighting.rs")
                    .annotation(
                        AnnotationKind::Primary
                            .span(553..563)
                            .label("one type is more general than the other"),
                    )
                    .annotation(
                        AnnotationKind::Context
                            .span(547..552)
                            .label("arguments to this function are incorrect"),
                    ),
            )
            .element(Level::NOTE.pre_styled_title(&title)),
        Group::new()
            .element(Level::NOTE.title("function defined here"))
            .element(
                Snippet::source(source)
                    .fold(true)
                    .path("$DIR/highlighting.rs")
                    .annotation(AnnotationKind::Context.span(200..333).label(""))
                    .annotation(AnnotationKind::Primary.span(194..199)),
            ),
    ];

    let renderer = Renderer::styled().anonymized_line_numbers(true);
    anstream::println!("{}", renderer.render(message));
}
