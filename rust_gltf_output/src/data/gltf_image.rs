use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// glTF Image
///
/// Describes an image used by a texture.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct GltfImage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_view: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

impl Default for GltfImage {
    fn default() -> Self {
        GltfImage {
            uri: None,
            mime_type: None,
            buffer_view: None,
            name: None,
            extensions: None,
            extras: None,
        }
    }
}
