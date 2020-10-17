// std
use std::fs::{read, write};
use std::path::Path;

// extern
use clap::{App, Arg, SubCommand};
use hyperbuild::{hyperbuild_copy, Cfg};

use walkdir::WalkDir;

use serde_json::to_writer_pretty;

// local
use web_utils::Index;

static VALID_INDEX_EXTENSIONS: [&str; 4] = ["html", "htm", "js", "css"];

fn main() {
    let app = App::new("web-utils")
        .version("0.0.1")
        .author("Ethan Justice")
        .about("Utilities for web programming (JS, CSS, HTML).")
        .subcommand(
            SubCommand::with_name("index")
                .about("Index a directory, and generate a JSON file")
                .version("0.0.1")
                .author("Ethan Justice")
                .arg(
                    Arg::with_name("include-assets")
                        .short("a")
                        .help("Include CSS and JS files")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("INPUT")
                        .help("The directory to index.")
                        .index(1)
                        .required(true),
                ),
        )
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
        }
    } else if let Some(v) = app.subcommand_matches("index") {
        let dir = Path::new(v.value_of("INPUT").unwrap());
        if dir.exists() == true {
            if dir.is_dir() == true {
                let mut file_index = Vec::new();
                for entry in WalkDir::new(dir) {
                    let file = entry.expect("Couldn't read file, aborting...");
                    match file.path().extension() {
                        Some(ext) => {
                            if VALID_INDEX_EXTENSIONS.contains(&ext.to_string_lossy().as_ref()) {
                                file_index.push(format!(
                                    "{}",
                                    file.into_path().display().to_string().replace("\\", "/")
                                ));
                            }
                        }
                        None => {}
                    }
                }
                to_writer_pretty(
                    std::fs::File::create(dir.join("index.json")).expect("Failed to write to file"),
                    &Index { files: file_index },
                )
                .expect("Failed to write to file.");
            }
        }
    }
}
