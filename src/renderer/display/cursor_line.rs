use super::end_line::EndLine;

pub(crate) struct CursorLines<'a>(&'a str);

impl CursorLines<'_> {
    pub(crate) fn new(src: &str) -> CursorLines<'_> {
        CursorLines(src)
    }
}

impl<'a> Iterator for CursorLines<'a> {
    type Item = (&'a str, EndLine);

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            None
        } else {
            self.0
                .find('\n')
                .map(|x| {
                    let ret = if 0 < x {
                        if self.0.as_bytes()[x - 1] == b'\r' {
                            (&self.0[..x - 1], EndLine::Crlf)
                        } else {
                            (&self.0[..x], EndLine::Lf)
                        }
                    } else {
                        ("", EndLine::Lf)
                    };
                    self.0 = &self.0[x + 1..];
                    ret
                })
                .or_else(|| {
                    let ret = Some((self.0, EndLine::Eof));
                    self.0 = "";
                    ret
                })
        }
    }
}
