use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// glTF Scene
///
/// The root nodes of a scene.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct GltfScene {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

impl Default for GltfScene {
    fn default() -> Self {
        GltfScene {
            nodes: None,
            name: None,
            extensions: None,
            extras: None,
        }
    }
}
