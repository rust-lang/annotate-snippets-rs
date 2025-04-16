use annotate_snippets::{AnnotationKind, Group, Level, Renderer, Snippet};
use anstyle::Effects;

fn main() {
    let source = r#"// Make sure "highlighted" code is colored purple

//@ compile-flags: --error-format=human --color=always
//@ error-pattern:[35mfor<'a> [0m
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
}
"#;

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

    let message = Level::Error.message("mismatched types").id("E0308").group(
        Group::new()
            .element(
                Snippet::source(source)
                    .fold(true)
                    .origin("$DIR/highlighting.rs")
                    .annotation(
                        AnnotationKind::Primary
                            .span(589..599)
                            .label("one type is more general than the other"),
                    )
                    .annotation(
                        AnnotationKind::Context
                            .span(583..588)
                            .label("arguments to this function are incorrect"),
                    ),
            )
            .element(Level::Note.title(&title)),
    );

    let renderer = Renderer::styled().anonymized_line_numbers(true);
    anstream::println!("{}", renderer.render(message));
}
