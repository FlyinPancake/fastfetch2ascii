use std::path::PathBuf;

use clap::Parser;

fn main() {
    let args = CliArgs::parse();

    let mut input = std::fs::read_to_string(args.input).unwrap();

    let re = regex::Regex::new(r"[\$\d]+").unwrap();
    let mut last_color = "\x1b[0m".to_string();
    input = input
        .split("\n")
        .map(|line| {
            last_color.clone()
                + &re
                    .replace_all(line, |caps: &regex::Captures| {
                        // println!("{:?}", caps);
                        let colors: Vec<String> = caps
                            .get(0)
                            .unwrap()
                            .as_str()
                            .chars()
                            .enumerate()
                            .filter_map(|(ii, el)| {
                                if ii % 2 == 1 {
                                    let idx: usize =
                                        (el.to_digit(10).unwrap() - 1).try_into().unwrap();
                                    Some(args.colors[idx].to_string())
                                } else {
                                    None
                                }
                            })
                            .collect();
                        let color = colors.join(";");
                        // let color = args.colors[color as usize % args.colors.len()];
                        last_color = format!("\x1b[{}m", color);
                        last_color.clone()
                    })
                    .to_string()
                + "\x1b[0m"
        })
        .collect::<Vec<String>>()
        .join("\n");

    print!("{}", input);
}

#[derive(Parser)]
#[command(styles=get_styles())]
/// Convert a fastfetch distro logos to plain ascii
///
/// This program takes a fastfetch distro logo and converts it to plain ascii.
/// The output is returned to stdout.
struct CliArgs {
    /// Path to the input file
    input: PathBuf,
    /// Colors to use
    ///
    /// The colors to use in the output. The colors are specified as numbers separated by spaces.
    ///
    /// See https://en.wikipedia.org/wiki/ANSI_escape_code#colors for the color codes.
    ///
    /// Example: `1 2 3 4 5 6 7 8 9 10 11 12 13 14 15`
    colors: Vec<String>,
}

pub fn get_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .usage(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow))),
        )
        .header(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow))),
        )
        .literal(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .invalid(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .error(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .valid(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .placeholder(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::White))),
        )
}
