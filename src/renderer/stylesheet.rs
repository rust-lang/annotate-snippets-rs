use anstyle::Style;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Stylesheet {
    pub(crate) error: Style,
    pub(crate) warning: Style,
    pub(crate) info: Style,
    pub(crate) note: Style,
    pub(crate) help: Style,
    pub(crate) line_no: Style,
    pub(crate) emphasis: Style,
    pub(crate) none: Style,
    pub(crate) context: Style,
}

impl Default for Stylesheet {
    fn default() -> Self {
        Self::plain()
    }
}

impl Stylesheet {
    pub(crate) const fn plain() -> Self {
        Self {
            error: Style::new(),
            warning: Style::new(),
            info: Style::new(),
            note: Style::new(),
            help: Style::new(),
            line_no: Style::new(),
            emphasis: Style::new(),
            none: Style::new(),
            context: Style::new(),
        }
    }
}
