use serde::{Deserialize, Deserializer, Serialize};
use std::ops::Range;

use annotate_snippets::renderer::DEFAULT_TERM_WIDTH;
use annotate_snippets::{Annotation, Level, Message, Renderer, Snippet};

#[derive(Deserialize)]
pub(crate) struct Fixture<'a> {
    #[serde(default)]
    pub(crate) renderer: RendererDef,
    #[serde(borrow)]
    pub(crate) message: MessageDef<'a>,
}

#[derive(Deserialize)]
pub struct MessageDef<'a> {
    #[serde(with = "LevelDef")]
    pub level: Level,
    #[serde(borrow)]
    pub title: &'a str,
    #[serde(default)]
    #[serde(borrow)]
    pub id: Option<&'a str>,
    #[serde(default)]
    #[serde(borrow)]
    pub footer: Vec<MessageDef<'a>>,
    #[serde(deserialize_with = "deserialize_snippets")]
    #[serde(borrow)]
    pub snippets: Vec<Snippet<'a>>,
}

impl<'a> From<MessageDef<'a>> for Message<'a> {
    fn from(val: MessageDef<'a>) -> Self {
        let MessageDef {
            level,
            title,
            id,
            footer,
            snippets,
        } = val;
        let mut message = level.title(title);
        if let Some(id) = id {
            message = message.id(id);
        }
        message = message.snippets(snippets);
        message = message.footers(footer.into_iter().map(Into::into));
        message
    }
}

fn deserialize_snippets<'de, D>(deserializer: D) -> Result<Vec<Snippet<'de>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper<'a>(
        #[serde(with = "SnippetDef")]
        #[serde(borrow)]
        SnippetDef<'a>,
    );

    let v = Vec::deserialize(deserializer)?;
    Ok(v.into_iter().map(|Wrapper(a)| a.into()).collect())
}

#[derive(Deserialize)]
pub struct SnippetDef<'a> {
    #[serde(borrow)]
    pub source: &'a str,
    pub line_start: usize,
    #[serde(borrow)]
    pub origin: Option<&'a str>,
    #[serde(deserialize_with = "deserialize_annotations")]
    #[serde(borrow)]
    pub annotations: Vec<Annotation<'a>>,
    #[serde(default)]
    pub fold: bool,
}

impl<'a> From<SnippetDef<'a>> for Snippet<'a> {
    fn from(val: SnippetDef<'a>) -> Self {
        let SnippetDef {
            source,
            line_start,
            origin,
            annotations,
            fold,
        } = val;
        let mut snippet = Snippet::source(source).line_start(line_start).fold(fold);
        if let Some(origin) = origin {
            snippet = snippet.origin(origin);
        }
        snippet = snippet.annotations(annotations);
        snippet
    }
}

fn deserialize_annotations<'de, D>(deserializer: D) -> Result<Vec<Annotation<'de>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper<'a>(#[serde(borrow)] AnnotationDef<'a>);

    let v = Vec::deserialize(deserializer)?;
    Ok(v.into_iter().map(|Wrapper(a)| a.into()).collect())
}

#[derive(Serialize, Deserialize)]
pub struct AnnotationDef<'a> {
    pub range: Range<usize>,
    #[serde(borrow)]
    pub label: &'a str,
    #[serde(with = "LevelDef")]
    pub level: Level,
}

impl<'a> From<AnnotationDef<'a>> for Annotation<'a> {
    fn from(val: AnnotationDef<'a>) -> Self {
        let AnnotationDef {
            range,
            label,
            level,
        } = val;
        level.span(range).label(label)
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct LabelDef<'a> {
    #[serde(with = "LevelDef")]
    pub(crate) level: Level,
    #[serde(borrow)]
    pub(crate) label: &'a str,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
#[serde(remote = "Level")]
enum LevelDef {
    Error,
    Warning,
    Info,
    Note,
    Help,
}

#[derive(Default, Deserialize)]
pub struct RendererDef {
    #[serde(default)]
    anonymized_line_numbers: bool,
    #[serde(default)]
    term_width: Option<usize>,
}

impl From<RendererDef> for Renderer {
    fn from(val: RendererDef) -> Self {
        let RendererDef {
            anonymized_line_numbers,
            term_width,
        } = val;
        Renderer::plain()
            .anonymized_line_numbers(anonymized_line_numbers)
            .term_width(term_width.unwrap_or(DEFAULT_TERM_WIDTH))
    }
}
