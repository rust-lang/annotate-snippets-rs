use annotate_snippets::renderer::{AnsiColor, Color, Style};
use difference::{Changeset, Difference};

const GREEN: Style = AnsiColor::Green.on_default();
pub fn get_diff(left: &str, right: &str) -> String {
    let mut output = String::new();

    let Changeset { diffs, .. } = Changeset::new(left, right, "\n");

    for i in 0..diffs.len() {
        match diffs[i] {
            Difference::Same(ref x) => {
                output += &format!(" {}\n", x);
            }
            Difference::Add(ref x) => {
                match diffs[i - 1] {
                    Difference::Rem(ref y) => {
                        output += &format!("{}+{}", GREEN.render(), GREEN.render_reset());
                        let Changeset { diffs, .. } = Changeset::new(y, x, " ");
                        for c in diffs {
                            match c {
                                Difference::Same(ref z) => {
                                    output += &format!(
                                        "{}{}{} ",
                                        GREEN.render(),
                                        z.as_str(),
                                        GREEN.render_reset()
                                    );
                                }
                                Difference::Add(ref z) => {
                                    let black_on_green = Style::new()
                                        .bg_color(Some(Color::Ansi(AnsiColor::Green)))
                                        .fg_color(Some(Color::Ansi(AnsiColor::Black)));
                                    output += &format!(
                                        "{}{}{} ",
                                        black_on_green.render(),
                                        z.as_str(),
                                        black_on_green.render_reset()
                                    );
                                }
                                _ => (),
                            }
                        }
                        output += "\n";
                    }
                    _ => {
                        output += &format!(
                            "+{}{}{}\n",
                            GREEN.render(),
                            x.as_str(),
                            GREEN.render_reset()
                        );
                    }
                };
            }
            Difference::Rem(ref x) => {
                let red = AnsiColor::Red.on_default();
                output += &format!("-{}{}{}\n", red.render(), x.as_str(), red.render_reset());
            }
        }
    }
    output
}
