use display_list::DisplayList;
use formatted_display_list::FormattedDisplayList;
use snippet::Snippet;

pub fn format_snippet(snippet: Snippet) -> String {
    let dl = DisplayList::from(snippet);
    let fdl = FormattedDisplayList::from(dl);
    format!("{}", fdl)
}
