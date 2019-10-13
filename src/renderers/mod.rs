pub mod ascii_default;

use crate::DisplayList;
use std::io::Write;

pub trait Renderer {
    fn fmt(&self, w: &mut impl Write, dl: &DisplayList) -> std::io::Result<()>;
}
