#[test]
fn custom_error() {
    let target = "custom_error";
    let expected = snapbox::file!["../examples/custom_error.svg": TermSvg];
    assert_example(target, expected);
}

#[test]
fn custom_level() {
    let target = "custom_level";
    let expected = snapbox::file!["../examples/custom_level.svg": TermSvg];
    assert_example(target, expected);
}

#[test]
fn expected_type() {
    let target = "expected_type";
    let expected = snapbox::file!["../examples/expected_type.svg": TermSvg];
    assert_example(target, expected);
}

#[test]
fn footer() {
    let target = "footer";
    let expected = snapbox::file!["../examples/footer.svg": TermSvg];
    assert_example(target, expected);
}

#[test]
fn format() {
    let target = "format";
    let expected = snapbox::file!["../examples/format.svg": TermSvg];
    assert_example(target, expected);
}

#[test]
fn highlight_source() {
    let target = "highlight_source";
    let expected = snapbox::file!["../examples/highlight_source.svg": TermSvg];
    assert_example(target, expected);
}

#[test]
fn highlight_title() {
    let target = "highlight_title";
    let expected = snapbox::file!["../examples/highlight_title.svg": TermSvg];
    assert_example(target, expected);
}

#[test]
fn multislice() {
    let target = "multislice";
    let expected = snapbox::file!["../examples/multislice.svg": TermSvg];
    assert_example(target, expected);
}

#[track_caller]
fn assert_example(target: &str, expected: snapbox::Data) {
    let bin_path = snapbox::cmd::compile_example(target, ["--features=testing-colors"]).unwrap();
    snapbox::cmd::Command::new(bin_path)
        .env("CLICOLOR_FORCE", "1")
        .assert()
        .success()
        .stdout_eq(expected.raw());
}
