use serde::{Deserialize, Serialize};
#[derive(Serialize, Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    Active,
    Done,
}
