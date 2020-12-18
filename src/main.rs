// std
use std::path::Path;

// extern
use clap::{App, Arg, SubCommand};

// local
use web_utils::{index, optimise};

fn main() {
    let app = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("index")
                .about("Index a directory, and generate a JSON file")
                .version("0.0.1")
                .author("Ethan Justice")
                .arg(
                    Arg::with_name("INPUT")
                        .help("The file or directory to use.")
                        .required(true),
                )
                .arg(
                    Arg::with_name("include-assets")
                        .long("include-assets")
                        .short("a")
                        .help("Include CSS and JS files")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("output")
                        .long("output")
                        .short("o")
                        .help("Specifiy a path for the output file")
                        .takes_value(true)
                        .required(false),
                )
                .arg(
                    Arg::with_name("accept")
                        .long("accept")
                        .help("Specify file types to include.")
                        .takes_value(true)
                        .required(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("optimise")
                .about("Performs optimisations on a specified file or directory")
                .version("0.0.1")
                .author("Ethan Justice")
                .arg(
                    Arg::with_name("INPUT")
                        .help("The file or directory to use.")
                        .required(true),
                )
                .arg(
                    Arg::with_name("minify")
                        .short("m")
                        .long("minify")
                        .help("Only minify the specified items.")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("inline")
                        .short("i")
                        .long("inline")
                        .help("Only inline the specified items.")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("recursive")
                        .short("r")
                        .long("recursive")
                        .help("Recursively optimise a directory")
                        .takes_value(false),
                ),
        )
        .get_matches();

    if let Some(v) = app.subcommand_matches("optimise") {
        optimise(&Path::new(v.value_of("INPUT").unwrap()));
    } else if let Some(v) = app.subcommand_matches("index") {
        index(
            &Path::new(v.value_of("INPUT").unwrap()),
            v.is_present("include-assets"),
            v.value_of("output"),
        );
    }
}
