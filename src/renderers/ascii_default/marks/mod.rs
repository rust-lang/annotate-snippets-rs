pub enum MarkKind {
    Vertical,
    Horizontal,
    DownRight,
    UpRight,
    UpLeft,
}

#[cfg(not(feature = "unicode"))]
impl MarkKind {
    pub fn get(t: MarkKind) -> char {
        match t {
            MarkKind::Vertical => '|',
            MarkKind::Horizontal => '-',
            MarkKind::DownRight => '/',
            MarkKind::UpRight => '|',
            MarkKind::UpLeft => '^',
        }
    }
}

#[cfg(feature = "unicode")]
impl MarkKind {
    pub fn get(t: MarkKind) -> char {
        match t {
            MarkKind::Vertical => '│',
            MarkKind::Horizontal => '─',
            MarkKind::DownRight => '┌',
            MarkKind::UpRight => '└',
            MarkKind::UpLeft => '┘',
        }
    }
}
