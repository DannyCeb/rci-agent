use serde::{Deserialize, Serialize};

use super::{Lang, Output, Step};

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Task {
    pub lang: Lang,
    pub steps: Vec<Step>,
    pub source: String,
    pub output: Option<Output>,
}
