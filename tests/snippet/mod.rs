extern crate annotate_snippets;
extern crate serde;

use self::serde::de::{Deserialize, Deserializer};

use self::annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet, TitleAnnotation};

#[derive(Deserialize)]
#[serde(remote = "Snippet")]
pub struct SnippetDef {
    #[serde(with = "SliceDef")]
    pub slice: Slice,
    #[serde(deserialize_with = "deserialize_annotations")]
    pub annotations: Vec<Annotation>,
    #[serde(deserialize_with = "deserialize_title_annotation")]
    pub title: Option<TitleAnnotation>,
    pub fold: Option<bool>,
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

fn deserialize_title_annotation<'de, D>(
    deserializer: D,
) -> Result<Option<TitleAnnotation>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper(#[serde(with = "TitleAnnotationDef")] TitleAnnotation);

    Option::<Wrapper>::deserialize(deserializer)
        .map(|opt_wrapped: Option<Wrapper>| opt_wrapped.map(|wrapped: Wrapper| wrapped.0))
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Slice")]
pub struct SliceDef {
    pub source: String,
    pub line_start: usize,
    pub origin: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Annotation")]
pub struct AnnotationDef {
    pub range: (usize, usize),
    pub label: String,
    #[serde(with = "AnnotationTypeDef")]
    pub annotation_type: AnnotationType,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "TitleAnnotation")]
pub struct TitleAnnotationDef {
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
}
