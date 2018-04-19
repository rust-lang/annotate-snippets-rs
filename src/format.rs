use display_list::DisplayList;
use snippet::Snippet;

pub fn format_snippet(snippet: Snippet) -> String {
    let dl = DisplayList::from(snippet);
    format!("{}", dl)
}
