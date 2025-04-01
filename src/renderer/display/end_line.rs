#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum EndLine {
    Eof,
    Lf,
    Crlf,
}

impl EndLine {
    /// The number of characters this line ending occupies in bytes.
    pub(crate) fn len(self) -> usize {
        match self {
            EndLine::Eof => 0,
            EndLine::Lf => 1,
            EndLine::Crlf => 2,
        }
    }
}
