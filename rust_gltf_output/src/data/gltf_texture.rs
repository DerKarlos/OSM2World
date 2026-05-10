use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// glTF Texture
///
/// A texture and its sampler.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct GltfTexture {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampler: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

impl Default for GltfTexture {
    fn default() -> Self {
        GltfTexture {
            sampler: None,
            source: None,
            name: None,
            extensions: None,
            extras: None,
        }
    }
}
