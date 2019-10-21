#[cfg(not(feature = "html"))]
pub mod ascii_default;
#[cfg(feature = "html")]
pub mod html;

#[cfg(feature = "html")]
use html::get_renderer as get_type_renderer;

#[cfg(not(feature = "html"))]
use ascii_default::get_renderer as get_type_renderer;

use crate::DisplayList;
use std::io::Write;

pub trait Renderer {
    fn fmt(&self, w: &mut impl Write, dl: &DisplayList) -> std::io::Result<()>;
}

pub fn get_renderer() -> impl Renderer {
    get_type_renderer()
}
