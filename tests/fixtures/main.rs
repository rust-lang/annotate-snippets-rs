mod deserialize;

use crate::deserialize::Fixture;
use annotate_snippets::{Message, Renderer};
use snapbox::data::DataFormat;
use snapbox::Data;
use std::error::Error;

fn main() {
    #[cfg(not(windows))]
    snapbox::harness::Harness::new("tests/fixtures/", setup, test)
        .select(["*/*.toml"])
        .action_env("SNAPSHOTS")
        .test();
}

fn setup(input_path: std::path::PathBuf) -> snapbox::harness::Case {
    let name = input_path.file_name().unwrap().to_str().unwrap().to_owned();
    let expected = input_path.with_extension("svg");
    snapbox::harness::Case {
        name,
        fixture: input_path,
        expected,
    }
}

fn test(input_path: &std::path::Path) -> Result<Data, Box<dyn Error>> {
    let src = std::fs::read_to_string(input_path)?;
    let (renderer, message): (Renderer, Message<'_>) =
        toml::from_str(&src).map(|a: Fixture| (a.renderer.into(), a.message.into()))?;
    let actual = renderer.render(message).to_string();
    Ok(Data::from(actual).coerce_to(DataFormat::TermSvg))
}
