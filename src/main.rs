// std

// extern
use clap::{App, Arg, SubCommand};

fn main() {
    let app = App::new("web-utils")
        .version("0.0.1")
        .author("Ethan Justice")
        .about("Utilities for web programming (JS, CSS, HTML).")
        .subcommand(
            SubCommand::with_name("optimise")
                .about("Performs optimisations on a specified file or directory")
                .version("0.0.1")
                .author("Ethan Justice")
                .arg(
                    Arg::with_name("minify")
                        .short("m")
                        .help("Only minify the specified items.")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("inline")
                        .short("i")
                        .help("Only inline the specified items.")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("recursive")
                        .short("r")
                        .help("Recursively optimise a directory")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("INPUT")
                        .help("The file or directory to use.")
                        .index(1)
                        .required(true),
                ),
        )
        .get_matches();

    if let Some(_v) = app.subcommand_matches("optimise") {
        println!("Called optimise");
    }
}
