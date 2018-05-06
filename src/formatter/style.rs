pub enum StyleClass {
    Error,
    Warning,
    Info,
    Note,
    Help,

    LineNo,
}

pub trait Style {
    fn paint(&self, text: String) -> String;
    fn bold(&self) -> Box<Style>;
}

pub trait Stylesheet {
    fn get_style(&self, class: StyleClass) -> Box<Style>;
}
