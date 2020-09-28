// std
use std::fs::{read, write};
use std::path::Path;

// extern
use clap::{App, Arg, SubCommand};
use hyperbuild::{hyperbuild_copy, Cfg};

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

    if let Some(v) = app.subcommand_matches("optimise") {
        let file_or_dir = Path::new(v.value_of("INPUT").unwrap());
        if file_or_dir.exists() == true {
            if file_or_dir.is_dir() == true {
            } else if file_or_dir.is_file() == true {
                if file_or_dir.extension().unwrap() == "html" {
                    write(
                        file_or_dir,
                        &hyperbuild_copy(
                            &mut read(file_or_dir).expect("Failed to read file."),
                            &Cfg { minify_js: false },
                        )
                        .unwrap(),
                    )
                    .expect("Failed to write to file.");
                }
            }
            println!("Called optimise");
        }
    }
}
