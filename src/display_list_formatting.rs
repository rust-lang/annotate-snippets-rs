use std::fmt;
use display_list::{DisplayAnnotationType, DisplayLine, DisplayMark,
                            DisplaySnippetType};

pub trait DisplayListFormatting {
    fn format_snippet_type(snippet_type: &DisplaySnippetType) -> String;

    fn format_inline_marks(inline_marks: &[DisplayMark], inline_marks_width: usize) -> String {
        format!(
            "{:>width$}",
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
        range: &(usize, usize),
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

    fn format_line(f: &mut fmt::Formatter, dl: &DisplayLine, lineno_width: usize, inline_marks_width: usize) -> fmt::Result {
        match dl {
            DisplayLine::Description {
                snippet_type,
                id,
                label,
            } => writeln!(f,
                "{}[{}]: {}",
                Self::format_snippet_type(&snippet_type),
                id,
                label
            ),
            DisplayLine::Origin { path, row, col } => {
                writeln!(f, "{}--> {}:{}:{}", " ".repeat(lineno_width), path, row, col)
            }
            DisplayLine::EmptySource => writeln!(f, "{} |", " ".repeat(lineno_width)),
            DisplayLine::Source {
                lineno,
                inline_marks,
                content,
                ..
            } => writeln!(f,
                "{:>width$} |{} {}",
                lineno,
                Self::format_inline_marks(&inline_marks, inline_marks_width),
                content,
                width = lineno_width,
            ),
            DisplayLine::Annotation {
                inline_marks,
                range,
                label,
                annotation_type,
            } => writeln!(f,
                "{} |{}{}",
                " ".repeat(lineno_width),
                Self::format_inline_marks(&inline_marks, inline_marks_width),
                Self::format_annotation_content(range, &label, &annotation_type),
            ),
            DisplayLine::Fold => writeln!(f, "...  |",),
        }
    }
}
