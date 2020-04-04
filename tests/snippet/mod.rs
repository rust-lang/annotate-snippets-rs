use serde::{Deserialize, Deserializer, Serialize};

use annotate_snippets::{
    display_list::FormatOptions,
    snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation},
};

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
    #[serde(deserialize_with = "deserialize_opt")]
    #[serde(default)]
    pub opt: FormatOptions,
    #[serde(deserialize_with = "deserialize_slices")]
    #[serde(borrow)]
    pub slices: Vec<Slice<'a>>,
}

impl<'a> Into<Snippet<'a>> for SnippetDef<'a> {
    fn into(self) -> Snippet<'a> {
        let SnippetDef {
            title,
            footer,
            opt,
            slices,
        } = self;
        Snippet {
            title,
            footer,
            slices,
            opt,
        }
    }
}

fn deserialize_opt<'de, D>(deserializer: D) -> Result<FormatOptions, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper(#[serde(with = "FormatOptionsDef")] FormatOptions);

    Wrapper::deserialize(deserializer).map(|w| w.0)
}

#[derive(Deserialize)]
#[serde(remote = "FormatOptions")]
pub struct FormatOptionsDef {
    pub color: bool,
    pub anonymized_line_numbers: bool,
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
