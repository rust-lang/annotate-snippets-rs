use crate::{Annotation, Level, Slice, Snippet, Span as _, SpanFormatter, WithLineNumber};
use std::cmp;

pub fn format<'d, Span: crate::Span>(
    snippet: &'d Snippet<'d, Span>,
    f: &dyn SpanFormatter<Span>,
) -> FormattedSnippet<'d, Span> {
    let mut lines = vec![];

    if let Some(title) = snippet.title {
        lines.push(DisplayLine::Raw(RawLine::Title { title }))
    }

    for slice in snippet.slices {
        format_into(&mut lines, slice, f);
    }

    FormattedSnippet { lines }
}

fn format_into<'d, Span: crate::Span>(
    lines: &mut Vec<DisplayLine<'d, Span>>,
    slice: &'d Slice<'d, Span>,
    f: &dyn SpanFormatter<Span>,
) {
    let mut this_line = f.first_line(&slice.span);

    if let Some(origin) = slice.origin {
        lines.push(DisplayLine::Raw(RawLine::Origin {
            path: origin,
            pos: Some((
                this_line.line_num,
                f.count_columns(
                    &slice.span,
                    &slice.span.slice(this_line.data.start()..slice.span.start()),
                ),
            )),
        }));

        // spacing line iff origin line present
        lines.push(DisplayLine::Source {
            lineno: None,
            inline_marks: vec![],
            line: SourceLine::Empty,
        })
    }

    // TODO: benchmark whether `retain` here benefits more than the allocation overhead
    let mut annotations: Vec<&Annotation<'_, _>> = slice.annotations.iter().collect();

    let mut process_line = |line: &WithLineNumber<Span::Subspan>| {
        let WithLineNumber {
            data: line,
            line_num,
        } = line;

        let mut annotations_here = vec![];
        let mut marks_here = vec![];

        annotations.retain(|&ann| {
            let level = ann.message.map(|m| m.level).unwrap_or(Level::Info);

            if line.start() <= ann.span.start() && ann.span.end() <= line.end() {
                // Annotation in this line
                annotations_here.push(ann);
                false
            } else if line.start() <= ann.span.start() && ann.span.start() <= line.end() {
                // Annotation starts in this line
                marks_here.push(Mark {
                    kind: MarkKind::Start,
                    level,
                });
                true
            } else if ann.span.start() < line.start() && line.end() < ann.span.end() {
                // Annotation goes through this line
                marks_here.push(Mark {
                    kind: MarkKind::Continue,
                    level,
                });
                true
            } else if ann.span.start() < line.start() && ann.span.end() <= line.end() {
                // Annotation ends on this line
                marks_here.push(Mark {
                    kind: MarkKind::Continue,
                    level,
                });
                annotations_here.push(ann);
                false
            } else {
                // Annotation starts on later line
                true
            }
        });

        lines.push(DisplayLine::Source {
            lineno: Some(*line_num),
            inline_marks: marks_here,
            line: SourceLine::Content { span: &slice.span, subspan: line.clone() },
        });

        for ann in annotations_here {
            let level = ann.message.map(|m| m.level).unwrap_or(Level::Info);

            let start_pos = cmp::max(ann.span.start(), line.start());
            let start = f.count_columns(&slice.span, &slice.span.slice(line.start()..start_pos));
            let len = f.count_columns(&slice.span, &slice.span.slice(start_pos..ann.span.end()));

            let marks_here = if ann.span.start() < line.start() {
                vec![Mark {
                    kind: MarkKind::Here,
                    level,
                }]
            } else {
                vec![]
            };

            lines.push(DisplayLine::Source {
                lineno: None,
                inline_marks: marks_here,
                line: SourceLine::Annotation {
                    message: ann.message,
                    underline: (start, len),
                },
            })
        }
    };

    process_line(&this_line);
    while let Some(line) = f.next_line(&slice.span, &this_line) {
        this_line = line;
        process_line(&this_line);
    }

    if !slice.footer.is_empty() {
        // spacing line iff footer lines follow
        lines.push(DisplayLine::Source {
            lineno: None,
            inline_marks: vec![],
            line: SourceLine::Empty,
        });

        for &message in slice.footer {
            lines.push(DisplayLine::Raw(RawLine::Message { message }))
        }
    }
}

mod types;
pub use types::{
    DisplayLine, Mark, MarkKind, RawLine, SourceLine, FormattedSnippet,
};
