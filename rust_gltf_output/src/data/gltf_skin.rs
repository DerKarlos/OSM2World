use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// glTF Skin
///
/// Skin data used for skeletal animations.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct GltfSkin {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

impl Default for GltfSkin {
    fn default() -> Self {
        GltfSkin {
            name: None,
            extensions: None,
            extras: None,
        }
    }
}
