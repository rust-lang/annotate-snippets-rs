use snippet::Snippet;

pub fn format_snippet(snippet: &Snippet) -> String {
    snippet.slice.source.clone()
}
