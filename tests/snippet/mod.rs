extern crate annotate_snippets;
extern crate serde;

use self::serde::de::{Deserialize, Deserializer};

use self::annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet};

#[derive(Deserialize)]
#[serde(remote = "Snippet")]
pub struct SnippetDef {
    #[serde(with = "SliceDef")]
    pub slice: Slice,
    #[serde(deserialize_with = "deserialize_annotations")]
    pub annotations: Vec<Annotation>,
    pub main_annotation_pos: Option<usize>,
    pub title_annotation_pos: Option<usize>,
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
    pub range: (Option<usize>, Option<usize>),
    pub label: Option<String>,
    pub id: Option<String>,
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
