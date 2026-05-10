use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// glTF Animation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct GltfAnimation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub samplers: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

impl Default for GltfAnimation {
    fn default() -> Self {
        GltfAnimation {
            channels: None,
            samplers: None,
            name: None,
            extensions: None,
            extras: None,
        }
    }
}
