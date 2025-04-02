use serde::Deserialize;
use std::ops::Range;

use annotate_snippets::renderer::DEFAULT_TERM_WIDTH;
use annotate_snippets::{
    level::Level, Annotation, AnnotationKind, Element, Group, Message, Patch, Renderer, Snippet,
};

#[derive(Deserialize)]
pub(crate) struct Fixture {
    #[serde(default)]
    pub(crate) renderer: RendererDef,
    pub(crate) message: MessageDef,
}

#[derive(Deserialize)]
pub struct MessageDef {
    pub level: LevelDef,
    pub title: String,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub sections: Vec<ElementDef>,
}

impl<'a> From<&'a MessageDef> for Message<'a> {
    fn from(val: &'a MessageDef) -> Self {
        let MessageDef {
            level,
            title,
            id,
            sections,
        } = val;
        let mut message = Level::from(level).message(title);
        if let Some(id) = id {
            message = message.id(id);
        }

        message = message.group(Group::new().elements(sections.iter().map(|s| match s {
            ElementDef::Title(title) => {
                Element::Title(Level::from(&title.level).title(&title.title))
            }
            ElementDef::Cause(cause) => Element::Cause(Snippet::from(cause)),
            ElementDef::Suggestion(suggestion) => Element::Suggestion(Snippet::from(suggestion)),
        })));
        message
    }
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum ElementDef {
    Title(TitleDef),
    Cause(SnippetAnnotationDef),
    Suggestion(SnippetPatchDef),
}

impl<'a> From<&'a ElementDef> for Element<'a> {
    fn from(val: &'a ElementDef) -> Self {
        match val {
            ElementDef::Title(title) => {
                Element::Title(Level::from(&title.level).title(&title.title))
            }
            ElementDef::Cause(cause) => Element::Cause(Snippet::from(cause)),
            ElementDef::Suggestion(suggestion) => Element::Suggestion(Snippet::from(suggestion)),
        }
    }
}

#[derive(Deserialize)]
pub struct TitleDef {
    pub title: String,
    pub level: LevelDef,
}

#[derive(Deserialize)]
pub struct SnippetAnnotationDef {
    pub(crate) origin: Option<String>,
    pub(crate) line_start: usize,
    pub(crate) source: String,
    pub(crate) annotations: Vec<AnnotationDef>,
    #[serde(default)]
    pub(crate) fold: bool,
}

impl<'a> From<&'a SnippetAnnotationDef> for Snippet<'a, Annotation<'a>> {
    fn from(val: &'a SnippetAnnotationDef) -> Self {
        let SnippetAnnotationDef {
            origin,
            line_start,
            source,
            annotations,
            fold,
        } = val;
        let mut snippet = Snippet::source(source).line_start(*line_start).fold(*fold);
        if let Some(origin) = origin {
            snippet = snippet.origin(origin);
        }
        snippet = snippet.annotations(annotations.iter().map(Into::into));
        snippet
    }
}

#[derive(Deserialize)]
pub struct AnnotationDef {
    pub range: Range<usize>,
    pub label: String,
    #[serde(with = "AnnotationKindDef")]
    pub kind: AnnotationKind,
}

impl<'a> From<&'a AnnotationDef> for Annotation<'a> {
    fn from(val: &'a AnnotationDef) -> Self {
        let AnnotationDef { range, label, kind } = val;
        kind.span(range.start..range.end).label(label)
    }
}

#[allow(dead_code)]
#[derive(Deserialize)]
#[serde(remote = "AnnotationKind")]
enum AnnotationKindDef {
    Primary,
    Context,
}

#[derive(Deserialize)]
pub struct SnippetPatchDef {
    pub(crate) origin: Option<String>,
    pub(crate) line_start: usize,
    pub(crate) source: String,
    pub(crate) patches: Vec<PatchDef>,
    #[serde(default)]
    pub(crate) fold: bool,
}

impl<'a> From<&'a SnippetPatchDef> for Snippet<'a, Patch<'a>> {
    fn from(val: &'a SnippetPatchDef) -> Self {
        let SnippetPatchDef {
            origin,
            line_start,
            source,
            patches,
            fold,
        } = val;
        let mut snippet = Snippet::source(source).line_start(*line_start).fold(*fold);
        if let Some(origin) = origin {
            snippet = snippet.origin(origin);
        }
        snippet = snippet.patches(patches.iter().map(Into::into));
        snippet
    }
}

#[derive(Deserialize)]
pub struct PatchDef {
    pub range: Range<usize>,
    pub replacement: String,
}

impl<'a> From<&'a PatchDef> for Patch<'a> {
    fn from(val: &'a PatchDef) -> Self {
        let PatchDef { range, replacement } = val;
        Patch::new(range.start..range.end, replacement)
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Deserialize)]
pub enum LevelDef {
    Error,
    Warning,
    Info,
    Note,
    Help,
}

impl<'a> From<&'a LevelDef> for Level<'a> {
    fn from(val: &'a LevelDef) -> Self {
        match val {
            LevelDef::Error => Level::ERROR,
            LevelDef::Warning => Level::WARNING,
            LevelDef::Info => Level::INFO,
            LevelDef::Note => Level::NOTE,
            LevelDef::Help => Level::HELP,
        }
    }
}

#[derive(Default, Deserialize)]
pub struct RendererDef {
    #[serde(default)]
    anonymized_line_numbers: bool,
    #[serde(default)]
    term_width: Option<usize>,
    #[serde(default)]
    color: bool,
}

impl From<RendererDef> for Renderer {
    fn from(val: RendererDef) -> Self {
        let RendererDef {
            anonymized_line_numbers,
            term_width,
            color,
        } = val;

        let renderer = if color {
            Renderer::styled()
        } else {
            Renderer::plain()
        };
        renderer
            .anonymized_line_numbers(anonymized_line_numbers)
            .term_width(term_width.unwrap_or(DEFAULT_TERM_WIDTH))
    }
}
