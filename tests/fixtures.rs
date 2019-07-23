mod diff;
mod snippet;

use crate::snippet::SnippetDef;
use annotate_snippets::display_list::DisplayList;
use annotate_snippets::formatter::DisplayListFormatter;
use annotate_snippets::snippet::Snippet;
use glob::glob;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use serde::Deserialize;

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    (f.read_to_string(&mut s))?;
    Ok(s.trim_end().to_string())
}

fn read_fixture<P: AsRef<Path>>(path: P) -> Result<Snippet, Box<Error>> {
    #[derive(Deserialize)]
    struct Wrapper(#[serde(with = "SnippetDef")] Snippet);

    let file = File::open(path)?;
    let u = serde_yaml::from_reader(file).map(|Wrapper(a)| a)?;
    Ok(u)
}

#[test]
fn test_fixtures() {
    for entry in glob("./tests/fixtures/no-color/**/*.yaml").expect("Failed to read glob pattern") {
        let p = entry.expect("Error while getting an entry");
        let path_in = p.to_str().expect("Can't print path");

        let path_out = path_in.replace(".yaml", ".txt");

        let snippet = read_fixture(path_in).expect("Failed to read file");
        let expected_out = read_file(&path_out).expect("Failed to read file");

        let dl = DisplayList::from(snippet);
        let dlf = DisplayListFormatter::new(true, false);
        let actual_out = dlf.format(&dl);
        println!("{}", expected_out);
        println!("{}", actual_out.trim_end());

        assert_eq!(
            expected_out,
            actual_out.trim_end(),
            "\n\n\nWhile parsing: {}\nThe diff is:\n\n\n{}\n\n\n",
            path_in,
            diff::get_diff(expected_out.as_str(), actual_out.as_str())
        );
    }
}
