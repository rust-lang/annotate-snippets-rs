use std::fmt;
use structs::display_list::{DisplayAnnotationType, DisplayLine, DisplayList, DisplayMark,
                            DisplaySnippetType};
struct DisplayListFormatting {}

impl DisplayListFormatting {
    fn format_snippet_type(snippet_type: &DisplaySnippetType) -> String {
        match snippet_type {
            DisplaySnippetType::Error => "error".to_string(),
            DisplaySnippetType::Warning => "warning".to_string(),
        }
    }

    fn format_inline_marks(inline_marks: &[DisplayMark], inline_marks_width: usize) -> String {
        format!(
            "{: >width$}",
            inline_marks
                .iter()
                .map(|mark| match mark {
                    DisplayMark::AnnotationThrough => "|",
                    DisplayMark::AnnotationStart => "/",
                })
                .collect::<Vec<&str>>()
                .join(""),
            width = inline_marks_width
        )
    }

    fn format_annotation_content(
        range: (usize, usize),
        label: &Option<String>,
        annotation_type: &DisplayAnnotationType,
    ) -> String {
        let label = label.clone().map_or("".to_string(), |l| format!(" {}", l));
        match annotation_type {
            DisplayAnnotationType::Error => format!(
                "{}{}{}",
                " ".repeat(range.0),
                "^".repeat(range.1 - range.0),
                label
            ),
            DisplayAnnotationType::Warning => format!(
                "{}{}{}",
                " ".repeat(range.0),
                "-".repeat(range.1 - range.0),
                label
            ),
            DisplayAnnotationType::MultilineStart => format!(
                "{}{}{}",
                "_".repeat(range.0),
                "^".repeat(range.1 - range.0),
                label
            ),
            DisplayAnnotationType::MultilineEnd => format!(
                "{}{}{}",
                "_".repeat(range.0),
                "^".repeat(range.1 - range.0),
                label
            ),
        }
    }

    pub fn format_line(dl: DisplayLine, lineno_width: usize, inline_marks_width: usize) -> String {
        match dl {
            DisplayLine::Description {
                snippet_type,
                id,
                label,
            } => format!(
                "{}[{}]: {}",
                DisplayListFormatting::format_snippet_type(&snippet_type),
                id,
                label
            ),
            DisplayLine::Origin { path, row, col } => format!(
                "{}--> {}:{}:{}",
                " ".repeat(lineno_width),
                path,
                row,
                col
            ),
            DisplayLine::EmptySource => format!(
                "{} |",
                " ".repeat(lineno_width)
            ),
            DisplayLine::Source {
                lineno,
                inline_marks,
                content,
                ..
            } => format!(
                "{:>width$} |{} {}",
                lineno,
                DisplayListFormatting::format_inline_marks(&inline_marks, inline_marks_width),
                content,
                width = lineno_width,
            ),
//             FormattedDisplayLine::Annotation {
//                 lineno,
//                 inline_marks,
//                 content,
//             } => write!(f, "{} |{}{}", lineno, inline_marks, content),
//             FormattedDisplayLine::Fold => write!(f, "...  |"),
            DisplayLine::Annotation {
                inline_marks,
                range,
                label,
                annotation_type,
            } => format!(
                "{} |{}{}",
                " ".repeat(lineno_width),
                DisplayListFormatting::format_inline_marks(&inline_marks, inline_marks_width),
                DisplayListFormatting::format_annotation_content(range, &label, &annotation_type),
            ),
            DisplayLine::Fold => format!(
                "...  |",
            ),
        }
    }
}

impl fmt::Display for DisplayList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lineno_width = self.body.iter().fold(0, |max, line| match line {
            DisplayLine::Source { lineno, .. } => {
                let width = lineno.to_string().len();
                if width > max {
                    width
                } else {
                    max
                }
            }
            _ => max,
        });
        let inline_marks_width = self.body.iter().fold(0, |max, line| match line {
            DisplayLine::Source { inline_marks, .. } => {
                let width = inline_marks.len();
                if width > max {
                    width + 1
                } else {
                    max
                }
            }
            _ => max,
        });
        let body = self.body
            .clone()
            .into_iter()
            .map(|line| DisplayListFormatting::format_line(line, lineno_width, inline_marks_width))
            .collect::<Vec<String>>();
        // let fdl = FormattedDisplayList { body };
        write!(f, "{}", body.join("\n"))
    }
}
