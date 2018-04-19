#[derive(Debug)]
pub struct Snippet {
    pub slice: Slice,
    pub annotations: Vec<Annotation>,
    pub main_annotation_pos: Option<usize>,
    pub title_annotation_pos: Option<usize>,
}

impl Snippet {
    pub fn new(
        slice: Slice,
        annotations: Vec<Annotation>,
        main_annotation_pos: Option<usize>,
        title_annotation_pos: Option<usize>,
    ) -> Snippet {
        Snippet {
            slice,
            annotations,
            main_annotation_pos,
            title_annotation_pos,
        }
    }
}

#[derive(Debug)]
pub struct Slice {
    pub source: String,
    pub line_start: usize,
    pub origin: Option<String>,
}

impl Slice {
    pub fn new(source: String, line_start: usize, origin: Option<String>) -> Slice {
        Slice {
            source,
            line_start,
            origin,
        }
    }
}

#[derive(Debug)]
pub enum AnnotationType {
    Error,
    Warning,
}

#[derive(Debug)]
pub struct Annotation {
    pub range: (Option<usize>, Option<usize>),
    pub label: Option<String>,
    pub id: Option<String>,
    pub annotation_type: AnnotationType,
}

impl Annotation {
    pub fn new(
        start: Option<usize>,
        end: Option<usize>,
        label: Option<String>,
        id: Option<String>,
        annotation_type: AnnotationType,
    ) -> Annotation {
        Annotation {
            range: (start, end),
            label,
            id,
            annotation_type,
        }
    }
}
