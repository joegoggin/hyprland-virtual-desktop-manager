use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Monitor {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub min_workspace_id: u64,
    pub max_workspace_id: u64,
}

impl Monitor {
    pub fn new(
        id: impl Into<u64>,
        name: impl Into<String>,
        description: impl Into<String>,
        min_workspace_id: impl Into<u64>,
        max_workspace_id: impl Into<u64>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: description.into(),
            min_workspace_id: min_workspace_id.into(),
            max_workspace_id: max_workspace_id.into(),
        }
    }
}
