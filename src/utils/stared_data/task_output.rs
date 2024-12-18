use serde::{Deserialize, Serialize};
#[derive(Serialize, Debug, Deserialize, Clone)]
pub enum TaskOutput {
    Pending,
    Result(String),
}
