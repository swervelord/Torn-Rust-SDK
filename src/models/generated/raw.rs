use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct RawSelectionBundle {
    #[serde(flatten)]
    pub selections: BTreeMap<String, serde_json::Value>,
}

impl RawSelectionBundle {
    pub fn get(&self, selection: &str) -> Option<&serde_json::Value> {
        self.selections.get(selection)
    }
}
