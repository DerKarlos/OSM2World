use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// glTF Camera
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct GltfCamera {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

impl Default for GltfCamera {
    fn default() -> Self {
        GltfCamera {
            name: None,
            extensions: None,
            extras: None,
        }
    }
}
