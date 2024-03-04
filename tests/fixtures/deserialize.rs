use serde::{Deserialize, Deserializer, Serialize};

use annotate_snippets::{
    renderer::Margin, Annotation, AnnotationType, Renderer, Slice, Snippet, SourceAnnotation,
};

#[derive(Deserialize)]
pub struct Fixture<'a> {
    #[serde(default)]
    pub renderer: RendererDef,
    #[serde(borrow)]
    pub snippet: SnippetDef<'a>,
}

#[derive(Deserialize)]
pub struct SnippetDef<'a> {
    #[serde(deserialize_with = "deserialize_annotation")]
    #[serde(default)]
    #[serde(borrow)]
    pub title: Option<Annotation<'a>>,
    #[serde(deserialize_with = "deserialize_annotations")]
    #[serde(default)]
    #[serde(borrow)]
    pub footer: Vec<Annotation<'a>>,
    #[serde(deserialize_with = "deserialize_slices")]
    #[serde(borrow)]
    pub slices: Vec<Slice<'a>>,
}

impl<'a> From<SnippetDef<'a>> for Snippet<'a> {
    fn from(val: SnippetDef<'a>) -> Self {
        let SnippetDef {
            title,
            footer,
            slices,
        } = val;
        Snippet {
            title,
            footer,
            slices,
        }
    }
}

fn deserialize_slices<'de, D>(deserializer: D) -> Result<Vec<Slice<'de>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper<'a>(
        #[serde(with = "SliceDef")]
        #[serde(borrow)]
        Slice<'a>,
    );

    let v = Vec::deserialize(deserializer)?;
    Ok(v.into_iter().map(|Wrapper(a)| a).collect())
}

fn deserialize_annotation<'de, D>(deserializer: D) -> Result<Option<Annotation<'de>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper<'a>(
        #[serde(with = "AnnotationDef")]
        #[serde(borrow)]
        Annotation<'a>,
    );

    Option::<Wrapper>::deserialize(deserializer)
        .map(|opt_wrapped: Option<Wrapper>| opt_wrapped.map(|wrapped: Wrapper| wrapped.0))
}

fn deserialize_annotations<'de, D>(deserializer: D) -> Result<Vec<Annotation<'de>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper<'a>(
        #[serde(with = "AnnotationDef")]
        #[serde(borrow)]
        Annotation<'a>,
    );

    let v = Vec::deserialize(deserializer)?;
    Ok(v.into_iter().map(|Wrapper(a)| a).collect())
}

#[derive(Deserialize)]
#[serde(remote = "Slice")]
pub struct SliceDef<'a> {
    #[serde(borrow)]
    pub source: &'a str,
    pub line_start: usize,
    #[serde(borrow)]
    pub origin: Option<&'a str>,
    #[serde(deserialize_with = "deserialize_source_annotations")]
    #[serde(borrow)]
    pub annotations: Vec<SourceAnnotation<'a>>,
    #[serde(default)]
    pub fold: bool,
}

fn deserialize_source_annotations<'de, D>(
    deserializer: D,
) -> Result<Vec<SourceAnnotation<'de>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper<'a>(
        #[serde(with = "SourceAnnotationDef")]
        #[serde(borrow)]
        SourceAnnotation<'a>,
    );

    let v = Vec::deserialize(deserializer)?;
    Ok(v.into_iter().map(|Wrapper(a)| a).collect())
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "SourceAnnotation")]
pub struct SourceAnnotationDef<'a> {
    pub range: (usize, usize),
    #[serde(borrow)]
    pub label: &'a str,
    #[serde(with = "AnnotationTypeDef")]
    pub annotation_type: AnnotationType,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Annotation")]
pub struct AnnotationDef<'a> {
    #[serde(borrow)]
    pub id: Option<&'a str>,
    #[serde(borrow)]
    pub label: Option<&'a str>,
    #[serde(with = "AnnotationTypeDef")]
    pub annotation_type: AnnotationType,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
#[serde(remote = "AnnotationType")]
enum AnnotationTypeDef {
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
    #[serde(deserialize_with = "deserialize_margin")]
    #[serde(default)]
    margin: Option<Margin>,
}

impl From<RendererDef> for Renderer {
    fn from(val: RendererDef) -> Self {
        let RendererDef {
            anonymized_line_numbers,
            margin,
        } = val;
        Renderer::plain()
            .anonymized_line_numbers(anonymized_line_numbers)
            .margin(margin)
    }
}

fn deserialize_margin<'de, D>(deserializer: D) -> Result<Option<Margin>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper {
        whitespace_left: usize,
        span_left: usize,
        span_right: usize,
        label_right: usize,
        column_width: usize,
        max_line_len: usize,
    }

    Option::<Wrapper>::deserialize(deserializer).map(|opt_wrapped: Option<Wrapper>| {
        opt_wrapped.map(|wrapped: Wrapper| {
            let Wrapper {
                whitespace_left,
                span_left,
                span_right,
                label_right,
                column_width,
                max_line_len,
            } = wrapped;
            Margin::new(
                whitespace_left,
                span_left,
                span_right,
                label_right,
                column_width,
                max_line_len,
            )
        })
    })
}
