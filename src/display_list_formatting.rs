use display_list::{DisplayAnnotationPart, DisplayAnnotationType, DisplayLine, DisplayMark,
                   DisplayMarkType, DisplayTextFragment};
use std::fmt;

pub trait DisplayListFormatting {
    fn format_annotation_type(annotation_type: &DisplayAnnotationType) -> String {
        match annotation_type {
            DisplayAnnotationType::Error => "error".to_string(),
            DisplayAnnotationType::Warning => "warning".to_string(),
            DisplayAnnotationType::Info => "info".to_string(),
            DisplayAnnotationType::Note => "note".to_string(),
            DisplayAnnotationType::Help => "help".to_string(),
        }
    }

    fn get_inline_mark(inline_mark: &DisplayMark) -> String {
        let sigil = match inline_mark.mark_type {
            DisplayMarkType::AnnotationThrough => '|',
            DisplayMarkType::AnnotationStart => '/',
        };
        sigil.to_string()
    }

    fn format_inline_mark(inline_mark: &DisplayMark) -> String {
        Self::get_inline_mark(inline_mark)
    }

    fn format_inline_marks(inline_marks: &[DisplayMark], inline_marks_width: usize) -> String {
        format!(
            "{}{}",
            " ".repeat(inline_marks_width - inline_marks.len()),
            inline_marks
                .iter()
                .map(Self::format_inline_mark)
                .collect::<Vec<String>>()
                .join(""),
        )
    }

    fn get_source_annotation_marks(
        annotation_type: &DisplayAnnotationType,
        annotation_part: &DisplayAnnotationPart,
    ) -> (char, char) {
        let indent_char = match annotation_part {
            DisplayAnnotationPart::Singleline => ' ',
            DisplayAnnotationPart::MultilineStart => '_',
            DisplayAnnotationPart::MultilineEnd => '_',
        };
        let mark = match annotation_type {
            DisplayAnnotationType::Error => '^',
            DisplayAnnotationType::Warning => '-',
            DisplayAnnotationType::Info => '-',
            DisplayAnnotationType::Note => '-',
            DisplayAnnotationType::Help => '-',
        };
        return (indent_char, mark);
    }

    fn format_source_annotation_parts(
        annotation_type: &DisplayAnnotationType,
        indent_char: char,
        mark: char,
        range: &(usize, usize),
        lineno_width: usize,
    ) -> (String, String, String);

    fn format_label_line(_annotation_type: &DisplayAnnotationType, line: &str) -> String {
        return line.to_string();
    }

    fn format_source_annotation_lines(
        f: &mut fmt::Formatter,
        lineno_width: usize,
        inline_marks: String,
        range: &(usize, usize),
        label: &[DisplayTextFragment],
        annotation_type: &DisplayAnnotationType,
        annotation_part: &DisplayAnnotationPart,
    ) -> fmt::Result {
        let (indent_char, mark) =
            Self::get_source_annotation_marks(annotation_type, annotation_part);
        let (lineno, indent, pointer) = Self::format_source_annotation_parts(
            annotation_type,
            indent_char,
            mark,
            range,
            lineno_width,
        );
        if let Some((first, rest)) = Self::format_label(label)
            .lines()
            .collect::<Vec<&str>>()
            .split_first()
        {
            writeln!(
                f,
                "{}{}{}{} {}",
                lineno,
                inline_marks,
                indent,
                pointer,
                Self::format_label_line(annotation_type, *first),
            )?;
            for line in rest {
                writeln!(
                    f,
                    "{}{}{} {}",
                    lineno,
                    inline_marks,
                    " ".repeat(range.1),
                    Self::format_label_line(annotation_type, *line),
                )?;
            }
        } else {
            writeln!(f, "{}{}{}{}", lineno, inline_marks, indent, pointer,)?;
        }
        Ok(())
    }

    fn format_label(label: &[DisplayTextFragment]) -> String;

    fn format_line(
        f: &mut fmt::Formatter,
        dl: &DisplayLine,
        lineno_width: usize,
        inline_marks_width: usize,
    ) -> fmt::Result;
}
