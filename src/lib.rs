// std
use std::fs::{create_dir_all, read, read_to_string, write, File};
use std::path::Path;

// crates
use hyperbuild::{hyperbuild_copy, Cfg};
use min::css::minify_from_str;
use serde::Serialize;
use serde_json::to_writer_pretty;
use walkdir::WalkDir;

// local

static VALID_INDEX_EXTENSIONS: [&str; 2] = ["html", "htm"];
static VALID_INDEX_ASSET_EXTENSIONS: [&str; 2] = ["js", "css"];

/// Represents an index of .html, .htm, .js, and .css files
#[derive(Serialize)]
pub struct AssetIndex {
    pub files: Vec<String>,
}

pub fn optimise(file_or_dir: &Path) {
    if file_or_dir.exists() == true {
        if file_or_dir.is_dir() == true {
        } else if file_or_dir.is_file() == true {
            let extension = file_or_dir.extension().unwrap();
            if extension == "html" {
                write(
                    file_or_dir,
                    &hyperbuild_copy(
                        &mut read(file_or_dir).expect("Failed to read html file."),
                        &Cfg { minify_js: false },
                    )
                    .unwrap(),
                )
                .expect("Failed to write to file.");
            } else if extension == "css" {
                write(
                    file_or_dir,
                    minify_from_str(
                        read_to_string(file_or_dir)
                            .expect("Failed to read CSS file.")
                            .as_str(),
                    ),
                )
                .expect("Failed to write to file.");
            }
        }
    }
}

/// Indexes a directory and generates an index
pub fn index(dir: &Path, include_assets: bool, output: Option<&str>) {
    if dir.exists() == true {
        if dir.is_dir() == true {
            let mut file_index = Vec::new();
            for entry in WalkDir::new(dir) {
                let file = entry.expect("Couldn't read file, aborting...");
                match file.path().extension() {
                    Some(ext) => {
                        let s = ext.to_string_lossy();
                        if VALID_INDEX_EXTENSIONS.contains(&s.as_ref())
                            || (include_assets == true
                                && VALID_INDEX_ASSET_EXTENSIONS.contains(&s.as_ref()))
                        {
                            file_index.push(format!(
                                "{}",
                                file.into_path()
                                    .display()
                                    .to_string()
                                    .replace("\\", "/")
                                    .replace("./", "")
                            ));
                        }
                    }
                    None => {}
                }
            }
            let output_path = match output {
                Some(out) => {
                    let path = Path::new(out);
                    if path.is_dir() == false {
                        create_dir_all(path).expect("Failed to create directory.");
                    }

                    path
                }
                None => dir,
            };

            to_writer_pretty(
                File::create(output_path.join("index.json")).expect("Failed to write to file"),
                &AssetIndex { files: file_index },
            )
            .expect("Failed to write to file.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_css_optimisation() {}

    #[test]
    fn check_html_optimisation() {}

    #[test]
    fn check_indexing() {}
}
