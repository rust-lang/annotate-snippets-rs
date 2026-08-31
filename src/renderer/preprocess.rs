use alloc::borrow::Cow;
use alloc::vec::Vec;
use core::cmp::max;

use super::source_map::{AnnotatedLineInfo, SourceMap, SplicedLines, TrimmedPatch};
use crate::{Annotation, Element, Group, Message, Origin, Padding, Patch, Snippet};

pub(crate) struct Preprocessed<'a> {
    pub(crate) max_line_num: Option<usize>,
    pub(crate) report_primary_path: Option<&'a Cow<'a, str>>,
    pub(crate) groups: Vec<PreprocessedGroup<'a>>,
}

impl Preprocessed<'_> {
    pub(crate) fn preprocess<'a>(groups: &'a [Group<'a>]) -> Preprocessed<'a> {
        preprocess(groups)
    }
}

pub(crate) struct PreprocessedGroup<'a> {
    pub(crate) group: &'a Group<'a>,
    pub(crate) elements: Vec<PreprocessedElement<'a>>,
    pub(crate) primary_path: Option<&'a Cow<'a, str>>,
    pub(crate) max_depth: usize,
}

pub(crate) enum PreprocessedElement<'a> {
    Message(&'a Message<'a>),
    Cause(
        (
            &'a Snippet<'a, Annotation<'a>>,
            SourceMap<'a>,
            Vec<AnnotatedLineInfo<'a>>,
        ),
    ),
    Suggestion(
        (
            &'a Snippet<'a, Patch<'a>>,
            SourceMap<'a>,
            SplicedLines<'a>,
            DisplaySuggestion,
        ),
    ),
    Origin(&'a Origin<'a>),
    Padding(Padding),
}

fn preprocess<'a>(groups: &'a [Group<'a>]) -> Preprocessed<'a> {
    let mut max_line_num = None;
    let mut report_primary_path = None;
    let mut out = Vec::with_capacity(groups.len());
    for group in groups {
        let mut elements = Vec::with_capacity(group.elements.len());
        let mut primary_path = None;
        let mut max_depth = 0;
        for element in &group.elements {
            match element {
                Element::Message(message) => {
                    elements.push(PreprocessedElement::Message(message));
                }
                Element::Cause(cause) => {
                    let sm = SourceMap::new(&cause.source, cause.line_start);
                    let (depth, annotated_lines) =
                        sm.annotated_lines(cause.markers.clone(), cause.fold);

                    if cause.fold {
                        let end = cause
                            .markers
                            .iter()
                            .map(|a| a.span.end)
                            .max()
                            .unwrap_or(cause.source.len())
                            .min(cause.source.len());

                        if cause.line_numbering {
                            max_line_num = Some(max(
                                cause.line_start + newline_count(&cause.source[..end]),
                                max_line_num.unwrap_or(0),
                            ));
                        }
                    } else {
                        if cause.line_numbering {
                            max_line_num = Some(max(
                                cause.line_start + newline_count(&cause.source),
                                max_line_num.unwrap_or(0),
                            ));
                        }
                    }

                    if primary_path.is_none() {
                        primary_path = Some(cause.path.as_ref());
                    }
                    max_depth = max(depth, max_depth);
                    elements.push(PreprocessedElement::Cause((cause, sm, annotated_lines)));
                }
                Element::Suggestion(suggestion) => {
                    let sm = SourceMap::new(&suggestion.source, suggestion.line_start);
                    if let Some(spliced_lines) =
                        sm.splice_lines(suggestion.markers.clone(), suggestion.fold)
                    {
                        let display_suggestion = DisplaySuggestion::new(
                            &spliced_lines.complete,
                            &spliced_lines.patches,
                            &sm,
                        );

                        if suggestion.line_numbering {
                            if suggestion.fold {
                                if let Some(first) = spliced_lines.patches.first() {
                                    let (l_start, _) =
                                        sm.span_to_locations(first.original_span.clone());
                                    let nc = newline_count(&spliced_lines.complete);
                                    let sugg_max_line_num = match display_suggestion {
                                        DisplaySuggestion::Underline => l_start.line,
                                        DisplaySuggestion::Diff => {
                                            let file_lines = sm.span_to_lines(first.span.clone());
                                            file_lines
                                                .last()
                                                .map_or(l_start.line + nc, |line| line.line_index)
                                        }
                                        DisplaySuggestion::None => l_start.line + nc,
                                        DisplaySuggestion::Add => l_start.line + nc,
                                    };
                                    max_line_num =
                                        Some(max(sugg_max_line_num, max_line_num.unwrap_or(0)));
                                }
                            } else {
                                max_line_num = Some(max(
                                    suggestion.line_start + newline_count(&spliced_lines.complete),
                                    max_line_num.unwrap_or(0),
                                ));
                            }
                        }

                        elements.push(PreprocessedElement::Suggestion((
                            suggestion,
                            sm,
                            spliced_lines,
                            display_suggestion,
                        )));
                    }
                }
                Element::Origin(origin) => {
                    if primary_path.is_none() {
                        primary_path = Some(Some(&origin.path));
                    }
                    elements.push(PreprocessedElement::Origin(origin));
                }
                Element::Padding(padding) => {
                    elements.push(PreprocessedElement::Padding(padding.clone()));
                }
            }
        }
        let group = PreprocessedGroup {
            group,
            elements,
            primary_path: primary_path.unwrap_or_default(),
            max_depth,
        };
        if report_primary_path.is_none() && group.primary_path.is_some() {
            report_primary_path = group.primary_path;
        }
        out.push(group);
    }

    Preprocessed {
        max_line_num,
        report_primary_path,
        groups: out,
    }
}

fn newline_count(body: &str) -> usize {
    #[cfg(feature = "simd")]
    {
        // Trailing newlines do not count towards the number of lines
        // (this is based into `str::lines`)
        let trailing_newline = body.ends_with('\n');
        memchr::memchr_iter(b'\n', body.as_bytes()).count() - usize::from(trailing_newline)
    }
    #[cfg(not(feature = "simd"))]
    {
        body.lines().count().saturating_sub(1)
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum DisplaySuggestion {
    Underline,
    Diff,
    None,
    Add,
}

impl DisplaySuggestion {
    fn new(complete: &str, patches: &[TrimmedPatch<'_>], sm: &SourceMap<'_>) -> Self {
        let has_deletion = patches
            .iter()
            .any(|p| p.is_deletion(sm) || p.is_destructive_replacement(sm));
        let is_multiline = complete.lines().count() > 1;
        if has_deletion && !is_multiline {
            Self::Diff
        } else if patches.len() == 1
            && patches.first().is_some_and(|p| {
                p.replacement.ends_with('\n') && p.replacement.trim() == complete.trim()
            })
        {
            // We are adding a line(s) of code before code that was already there.
            Self::Add
        } else if (patches.len() != 1 || patches[0].replacement.trim() != complete.trim())
            && !is_multiline
        {
            Self::Underline
        } else {
            Self::None
        }
    }
}

#[cfg(test)]
mod test {
    use super::newline_count;

    #[test]
    fn ensure_newline_count_correct() {
        let source = r#"
                cargo-features = ["path-bases"]

                [package]
                name = "foo"
                version = "0.5.0"
                authors = ["wycats@example.com"]

                [dependencies]
                bar = { base = '^^not-valid^^', path = 'bar' }
            "#;
        assert_eq!(newline_count(source), 10);

        assert_eq!(newline_count(""), 0);

        assert_eq!(newline_count("one"), 0);

        assert_eq!(newline_count("one\n"), 0);

        assert_eq!(newline_count("one\ntwo"), 1);

        assert_eq!(newline_count("one\ntwo\n"), 1);

        assert_eq!(newline_count("one\n\n"), 1);

        assert_eq!(newline_count("one\r\ntwo\r\n"), 1);
    }
}
