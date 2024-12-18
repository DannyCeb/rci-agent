use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Step {
    Check,
    Test,
    Build,
    Deploy,
}
