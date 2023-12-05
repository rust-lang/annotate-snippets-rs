mod deserialize;
mod diff;

use crate::deserialize::Fixture;
use annotate_snippets::Renderer;
use annotate_snippets::Snippet;
use glob::glob;
use std::{error::Error, fs::File, io, io::prelude::*};

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    (f.read_to_string(&mut s))?;
    Ok(s.trim_end().to_string())
}

fn read_fixture(src: &str) -> Result<(Renderer, Snippet<'_>), Box<dyn Error>> {
    Ok(toml::from_str(src).map(|a: Fixture| (a.renderer.into(), a.snippet.into()))?)
}

#[test]
#[cfg(not(windows))] // HACK: Not working on windows due to a serde error
fn test_fixtures() {
    for entry in glob("./tests/fixtures/no-color/**/*.toml").expect("Failed to read glob pattern") {
        let p = entry.expect("Error while getting an entry");

        let path_in = p.to_str().expect("Can't print path");
        let path_out = path_in.replace(".toml", ".txt");

        let src = read_file(path_in).expect("Failed to read file");
        let (renderer, snippet) = read_fixture(&src).expect("Failed to read file");
        let expected_out = read_file(&path_out).expect("Failed to read file");

        let actual_out = renderer.render(snippet).to_string();
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
