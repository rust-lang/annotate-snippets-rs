mod diff;
mod snippet;

#[macro_use]
extern crate serde_derive;
extern crate annotate_snippets;
extern crate glob;
extern crate serde_yaml;

use annotate_snippets::format_snippet;
use annotate_snippets::snippet::Snippet;
use glob::glob;
use snippet::SnippetDef;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut f = try!(File::open(path));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    Ok(s.trim().to_string())
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
    for entry in glob("./tests/fixtures/**/*.yaml").expect("Failed to read glob pattern") {
        let p = entry.expect("Error while getting an entry");
        let path_in = p.to_str().expect("Can't print path");

        let path_out = path_in.replace(".yaml", ".txt");

        let snippet = read_fixture(path_in).expect("Failed to read file");
        let expected_out = read_file(&path_out).expect("Failed to read file");

        let actual_out = format_snippet(snippet);

        assert_eq!(
            expected_out,
            actual_out,
            "\n\n\nWhile parsing: {}\nThe diff is:\n\n\n{}\n\n\n",
            path_in,
            diff::get_diff(expected_out.as_str(), actual_out.as_str())
        );
    }
}
