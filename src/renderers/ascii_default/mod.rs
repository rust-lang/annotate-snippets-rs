pub mod styles;

use super::Renderer as RendererTrait;
use crate::display_list::line::DisplayLine;
use crate::display_list::line::DisplayRawLine;
use crate::DisplayList;
use std::io::Write;
use std::marker::PhantomData;
use styles::Style as StyleTrait;

pub struct Renderer<S: StyleTrait> {
    style: PhantomData<S>,
}

impl<S: StyleTrait> Renderer<S> {
    pub fn new() -> Self {
        Renderer { style: PhantomData }
    }

    pub fn fmt(&self, w: &mut impl Write, dl: &DisplayList) -> std::io::Result<()> {
        for line in &dl.body {
            self.fmt_line(w, line)?;
        }
        Ok(())
    }

    fn fmt_line(&self, w: &mut impl Write, line: &DisplayLine) -> std::io::Result<()> {
        match line {
            DisplayLine::Raw(l) => self.fmt_raw_line(w, l),
            _ => Ok(()),
        }
    }

    fn fmt_raw_line(
        &self,
        w: &mut impl std::io::Write,
        line: &DisplayRawLine,
    ) -> std::io::Result<()> {
        match line {
            DisplayRawLine::Origin { path, .. } => {
                let _lineno_max = 1;
                S::fmt(w, path)
                //write!(w, "{:>1$}", "", lineno_max)?;
                //write!(w, "--> {}", path)?;
                //if let Some(line) = pos.0 {
                //write!(w, ":{}", line)?;
                //}
                //w.write_char('\n')
            }
            _ => Ok(()),
        }
    }
}

impl<S: StyleTrait> RendererTrait for Renderer<S> {
    fn fmt(&self, w: &mut impl Write, dl: &DisplayList) -> std::io::Result<()> {
        Renderer::fmt(self, w, dl)
    }
}
