use serde::{Deserialize, Deserializer, Serialize};

use annotate_snippets::{
    display_list::FormatOptions,
    snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation},
};

#[derive(Deserialize)]
#[serde(remote = "Snippet")]
pub struct SnippetDef {
    #[serde(deserialize_with = "deserialize_annotation")]
    #[serde(default)]
    pub title: Option<Annotation>,
    #[serde(deserialize_with = "deserialize_annotations")]
    #[serde(default)]
    pub footer: Vec<Annotation>,
    #[serde(deserialize_with = "deserialize_opt")]
    #[serde(default)]
    pub opt: FormatOptions,
    #[serde(deserialize_with = "deserialize_slices")]
    pub slices: Vec<Slice>,
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

fn deserialize_slices<'de, D>(deserializer: D) -> Result<Vec<Slice>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper(#[serde(with = "SliceDef")] Slice);

    let v = Vec::deserialize(deserializer)?;
    Ok(v.into_iter().map(|Wrapper(a)| a).collect())
}

fn deserialize_annotation<'de, D>(deserializer: D) -> Result<Option<Annotation>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper(#[serde(with = "AnnotationDef")] Annotation);

    Option::<Wrapper>::deserialize(deserializer)
        .map(|opt_wrapped: Option<Wrapper>| opt_wrapped.map(|wrapped: Wrapper| wrapped.0))
}

fn deserialize_annotations<'de, D>(deserializer: D) -> Result<Vec<Annotation>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper(#[serde(with = "AnnotationDef")] Annotation);

    let v = Vec::deserialize(deserializer)?;
    Ok(v.into_iter().map(|Wrapper(a)| a).collect())
}

#[derive(Deserialize)]
#[serde(remote = "Slice")]
pub struct SliceDef {
    pub source: String,
    pub line_start: usize,
    pub origin: Option<String>,
    #[serde(deserialize_with = "deserialize_source_annotations")]
    pub annotations: Vec<SourceAnnotation>,
    #[serde(default)]
    pub fold: bool,
}

fn deserialize_source_annotations<'de, D>(
    deserializer: D,
) -> Result<Vec<SourceAnnotation>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper(#[serde(with = "SourceAnnotationDef")] SourceAnnotation);

    let v = Vec::deserialize(deserializer)?;
    Ok(v.into_iter().map(|Wrapper(a)| a).collect())
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "SourceAnnotation")]
pub struct SourceAnnotationDef {
    pub range: (usize, usize),
    pub label: String,
    #[serde(with = "AnnotationTypeDef")]
    pub annotation_type: AnnotationType,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Annotation")]
pub struct AnnotationDef {
    pub id: Option<String>,
    pub label: Option<String>,
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
