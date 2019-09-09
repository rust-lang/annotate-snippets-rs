use std::fmt;

#[derive(Debug, Clone)]
pub struct Annotation<'d> {
    pub label: &'d str,
}

impl<'d> fmt::Display for Annotation<'d> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label)
    }
}
