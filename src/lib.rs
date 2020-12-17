use serde::Serialize;

#[derive(Serialize)]
pub struct Index {
    pub files: Vec<String>,
}
