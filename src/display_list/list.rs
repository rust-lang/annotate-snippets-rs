use super::annotation::Annotation;
use super::line::{DisplayLine, DisplayRawLine, DisplaySourceLine};
use crate::slice::Slice;
use std::fmt;

#[derive(Debug, Clone)]
pub struct DisplayList<'d> {
    pub body: Vec<DisplayLine<'d>>,
}

fn get_header_pos(slice: &Slice) -> (Option<usize>, Option<usize>) {
    let line = slice.line_start;
    (line, None)
}

impl<'d> From<&Slice<'d>> for DisplayList<'d> {
    fn from(slice: &Slice<'d>) -> Self {
        let mut body = vec![];

        if let Some(path) = slice.origin {
            body.push(DisplayLine::Raw(DisplayRawLine::Origin {
                path,
                pos: get_header_pos(slice),
            }));
        }

        body.push(DisplayLine::Source {
            lineno: None,
            line: DisplaySourceLine::Empty,
        });

        let mut annotations = slice.annotations.iter();

        let mut current_annotation = annotations.next();
        let mut line_start_pos = 0;

        let mut i = slice.line_start.unwrap_or(1);
        for line in slice.source.lines() {
            let line_length = line.chars().count();
            body.push(DisplayLine::Source {
                lineno: Some(i),
                line: DisplaySourceLine::Content { text: line },
            });
            if let Some(annotation) = current_annotation {
                if annotation.range.0 >= line_start_pos
                    && annotation.range.1 <= line_start_pos + line_length
                {
                    body.push(DisplayLine::Source {
                        lineno: None,
                        line: DisplaySourceLine::Annotation {
                            annotation: Annotation {
                                label: annotation.label,
                            },
                            range: (
                                annotation.range.0 - line_start_pos,
                                annotation.range.1 - line_start_pos,
                            ),
                        },
                    });
                    current_annotation = annotations.next();
                }
            }
            line_start_pos += line_length;
            i += 1;
        }

        body.push(DisplayLine::Source {
            lineno: None,
            line: DisplaySourceLine::Empty,
        });

        DisplayList { body }
    }
}

fn digits(n: &usize) -> usize {
    let mut n = n.clone();
    let mut sum = 0;
    while n != 0 {
        n = n / 10;
        sum += 1;
    }
    sum
}

impl<'d> fmt::Display for DisplayList<'d> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lineno_max = self.body.iter().rev().find_map(|line| {
            if let DisplayLine::Source {
                lineno: Some(lineno),
                ..
            } = line
            {
                Some(digits(lineno))
            } else {
                None
            }
        });
        for line in &self.body {
            line.fmt(f, lineno_max)?
        }
        Ok(())
    }
}
