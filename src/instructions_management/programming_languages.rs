use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ProgrammingLanguage {
    Rust,
    Python,
}
