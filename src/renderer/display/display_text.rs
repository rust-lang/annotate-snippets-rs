/// An inline text fragment which any label is composed of.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DisplayTextFragment<'a> {
    pub(crate) content: &'a str,
    pub(crate) style: DisplayTextStyle,
}

/// A style for the `DisplayTextFragment` which can be visually formatted.
///
/// This information may be used to emphasis parts of the label.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum DisplayTextStyle {
    Regular,
    Emphasis,
}
