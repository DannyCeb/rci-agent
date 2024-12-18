use serde::{Deserialize, Serialize};
#[derive(Deserialize, Clone, Copy, Debug, Serialize)]
pub enum Lang {
    Rust,
    Csharp,
}
