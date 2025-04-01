/// Information whether the header is the initial one or a consequitive one
/// for multi-slice cases.
// TODO: private
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum DisplayHeaderType {
    /// Initial header is the first header in the snippet.
    Initial,

    /// Continuation marks all headers of following slices in the snippet.
    Continuation,
}
