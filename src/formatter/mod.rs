pub mod style;

use self::style::Stylesheet;

#[cfg(feature = "color")]
use crate::stylesheets::color::AnsiTermStylesheet;
use crate::stylesheets::no_color::NoColorStylesheet;

#[cfg(feature = "color")]
#[inline]
pub fn get_term_style(color: bool) -> Box<dyn Stylesheet> {
    if color {
        Box::new(AnsiTermStylesheet)
    } else {
        Box::new(NoColorStylesheet)
    }
}

#[cfg(not(feature = "color"))]
#[inline]
pub fn get_term_style(_color: bool) -> Box<dyn Stylesheet> {
    Box::new(NoColorStylesheet)
}
