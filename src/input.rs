use std::fmt;

pub trait DebugAndDisplay: fmt::Debug + fmt::Display {}
impl<T: ?Sized + fmt::Debug + fmt::Display> DebugAndDisplay for T {}

// Cannot derive Debug because we need to bound Span::Subspan
// so #[derive(Debug)] is manually expanded here (ugh)

/// Primary structure for annotation formatting.
///
/// # Examples
///
/// To produce the error annotation
///
/// ```text
/// error[E0277]: `std::sync::MutexGuard<'_, u32>` cannot be sent between threads safely
///   --> examples/nonsend_future.rs:23:5
///    |
/// 5  | fn is_send<T: Send>(t: T) {
///    |    -------    ---- required by this bound in `is_send`
/// ...
/// 23 |     is_send(foo());
///    |     ^^^^^^^ `std::sync::MutexGuard<'_, u32>` cannot be sent between threads safely
///    |
///    = help: within `impl std::future::Future`, the trait `std::marker::Send` is not implemented for `std::sync::MutexGuard<'_, u32>`
/// note: future does not implement `std::marker::Send` as this value is used across an await
///   --> examples/nonsend_future.rs:15:3
///    |
/// 14 |     let g = x.lock().unwrap();
///    |         - has type `std::sync::MutexGuard<'_, u32>`
/// 15 |     baz().await;
///    |     ^^^^^^^^^^^ await occurs here, with `g` maybe used later
/// 16 | }
///    | - `g` is later dropped here
/// ```
///
/// two snippets are used:
///
/// ```rust
/// # use annotate_snippets::*;
/// let first_snippet = Snippet {
///     title: Some(Title {
///         code: Some(&"E0277"),
///         message: Message {
///             text: &"`std::sync::MutexGuard<'_, u32>` cannot be sent between threads safely",
///             level: Level::Error,
///         },
///     }),
///     slices: &[Slice {
///         span: WithLineNumber {
///             data: "fn is_send<T: Send>(t: T) {\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n    is_send(foo());",
///             line_num: 5,
///         },
///         origin: Some(&"examples/nonsend_future.rs"),
///         annotations: &[
///             Annotation {
///                 span: 4..11,
///                 message: None,
///             },
///             Annotation {
///                 span: 14..18,
///                 message: Some(Message {
///                     text: &"required by this bound in `is_send`",
///                     level: Level::Info,
///                 })
///             },
///             Annotation {
///                 span: 67..74,
///                 message: Some(Message {
///                     text: &"`std::sync::MutexGuard<'_, u32>` cannot be sent between threads safely",
///                     level: Level::Error,
///                 })
///             },
///         ],
///         footer: &[Message {
///             text: &"within `impl std::future::Future`, the trait `std::marker::Send` is not implemented for `std::sync::MutexGuard<'_, u32>`",
///             level: Level::Help,
///         }],
///     }],
/// };
/// let second_snippet = Snippet {
///     title: Some(Title {
///         code: None,
///         message: Message {
///             text: &"future does not implement `std::marker::Send` as this value is used across an await",
///             level: Level::Note,
///         },
///     }),
///     slices: &[Slice {
///         span: WithLineNumber {
///             data: "    let g = x.lock().unwrap();\n    baz().await;\n}",
///             line_num: 14,
///         },
///         origin: Some(&"examples/nonsend_future.rs"),
///         annotations: &[
///             Annotation {
///                 span: 8..9,
///                 message: Some(Message {
///                     text: &"has type `std::sync::MutexGuard<'_, u32>`",
///                     level: Level::Info,
///                 }),
///             },
///             Annotation {
///                 span: 36..47,
///                 message: Some(Message {
///                     text: &"await occurs here, with `g` maybe used later",
///                     level: Level::Error,
///                 })
///             },
///             Annotation {
///                 span: 50..51,
///                 message: Some(Message {
///                     text: &"`g` is later dropped here",
///                     level: Level::Info,
///                 })
///             },
///         ],
///         footer: &[],
///     }],
/// };
/// ```
#[derive(Copy, Clone)]
pub struct Snippet<'s, Span: crate::Span> {
    pub title: Option<Title<'s>>,
    pub slices: &'s [Slice<'s, Span>],
}

// #[derive(Debug)]
impl<Span: crate::Span> fmt::Debug for Snippet<'_, Span>
where
    Span: fmt::Debug,
    Span::Subspan: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Snippet")
            .field("title", &self.title)
            .field("slices", &self.slices)
            .finish()
    }
}

/// Title line for an annotation snippet.
#[derive(Debug, Copy, Clone)]
pub struct Title<'s> {
    pub code: Option<&'s dyn DebugAndDisplay>,
    pub message: Message<'s>,
}

/// A slice of text with annotations.
#[derive(Copy, Clone)]
pub struct Slice<'s, Span: crate::Span> {
    pub span: Span,
    pub origin: Option<&'s dyn DebugAndDisplay>,
    pub annotations: &'s [Annotation<'s, Span>],
    pub footer: &'s [Message<'s>],
}

// #[derive(Debug)]
impl<Span: crate::Span> fmt::Debug for Slice<'_, Span>
where
    Span: fmt::Debug,
    Span::Subspan: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Slice")
            .field("span", &self.span)
            .field("origin", &self.origin)
            .field("annotations", &self.annotations)
            .field("footer", &self.footer)
            .finish()
    }
}

/// An annotation for some span.
pub struct Annotation<'s, Span: crate::Span> {
    pub span: Span::Subspan,
    pub message: Option<Message<'s>>,
}

// #[derive(Debug)]
impl<Span: crate::Span> fmt::Debug for Annotation<'_, Span>
where
    Span::Subspan: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Annotation")
            .field("span", &self.span)
            .field("message", &self.message)
            .finish()
    }
}

// #[derive(Copy)]
impl<Span: crate::Span> Copy for Annotation<'_, Span> where Span::Subspan: Copy {}

// #[derive(Clone)]
impl<Span: crate::Span> Clone for Annotation<'_, Span>
where
    Span::Subspan: Clone,
{
    fn clone(&self) -> Self {
        Annotation {
            span: self.span.clone(),
            message: self.message,
        }
    }
}

/// A message with an associated level.
#[derive(Debug, Copy, Clone)]
pub struct Message<'s> {
    pub text: &'s dyn DebugAndDisplay,
    pub level: Level,
}

/// A level of severity for an annotation message.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Level {
    /// Typically displayed using a red color.
    Error,
    /// Typically displayed using a blue color.
    Warning,
    Info,
    Note,
    Help,
}
