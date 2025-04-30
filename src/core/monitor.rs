use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Monitor {
    pub id: u64,
    pub name: String,
    pub description: String,
}

impl Monitor {
    pub fn new(
        id: impl Into<u64>,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: description.into(),
        }
    }
}
