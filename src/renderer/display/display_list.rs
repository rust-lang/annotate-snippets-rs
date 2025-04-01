use std::{
    cmp,
    fmt::{self, Display},
};

use crate::{
    renderer::{
        display::{
            constants::ANONYMIZED_LINE_NUM, display_annotations::DisplayAnnotationPart,
            display_line::DisplayLine,
        },
        styled_buffer::StyledBuffer,
        stylesheet::Stylesheet,
    },
    snippet,
};

use super::{display_set::DisplaySet, format_message};

/// List of lines to be displayed.
pub(crate) struct DisplayList<'a> {
    pub(crate) body: Vec<DisplaySet<'a>>,
    pub(crate) stylesheet: &'a Stylesheet,
    pub(crate) anonymized_line_numbers: bool,
}

impl PartialEq for DisplayList<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.body == other.body && self.anonymized_line_numbers == other.anonymized_line_numbers
    }
}

impl fmt::Debug for DisplayList<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DisplayList")
            .field("body", &self.body)
            .field("anonymized_line_numbers", &self.anonymized_line_numbers)
            .finish()
    }
}

impl Display for DisplayList<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lineno_width = self.body.iter().fold(0, |max, set| {
            set.display_lines.iter().fold(max, |max, line| match line {
                DisplayLine::Source { lineno, .. } => cmp::max(lineno.unwrap_or(0), max),
                _ => max,
            })
        });
        let lineno_width = if lineno_width == 0 {
            lineno_width
        } else if self.anonymized_line_numbers {
            ANONYMIZED_LINE_NUM.len()
        } else {
            ((lineno_width as f64).log10().floor() as usize) + 1
        };

        let multiline_depth = self.body.iter().fold(0, |max, set| {
            set.display_lines.iter().fold(max, |max2, line| match line {
                DisplayLine::Source { annotations, .. } => cmp::max(
                    annotations.iter().fold(max2, |max3, line| {
                        cmp::max(
                            match line.annotation_part {
                                DisplayAnnotationPart::Standalone => 0,
                                DisplayAnnotationPart::LabelContinuation => 0,
                                DisplayAnnotationPart::MultilineStart(depth) => depth + 1,
                                DisplayAnnotationPart::MultilineEnd(depth) => depth + 1,
                            },
                            max3,
                        )
                    }),
                    max,
                ),
                _ => max2,
            })
        });
        let mut buffer = StyledBuffer::new();
        for set in self.body.iter() {
            self.format_set(set, lineno_width, multiline_depth, &mut buffer)?;
        }
        write!(f, "{}", buffer.render(self.stylesheet)?)
    }
}

impl<'a> DisplayList<'a> {
    pub(crate) fn new(
        message: snippet::Message<'a>,
        stylesheet: &'a Stylesheet,
        anonymized_line_numbers: bool,
        term_width: usize,
    ) -> DisplayList<'a> {
        let body = format_message(message, term_width, anonymized_line_numbers, true);

        Self {
            body,
            stylesheet,
            anonymized_line_numbers,
        }
    }

    fn format_set(
        &self,
        set: &DisplaySet<'_>,
        lineno_width: usize,
        multiline_depth: usize,
        buffer: &mut StyledBuffer,
    ) -> fmt::Result {
        for line in &set.display_lines {
            set.format_line(
                line,
                lineno_width,
                multiline_depth,
                self.stylesheet,
                self.anonymized_line_numbers,
                buffer,
            )?;
        }
        Ok(())
    }
}
