use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// glTF Buffer
///
/// Describes a buffer containing binary data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct GltfBuffer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    pub byte_length: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

impl GltfBuffer {
    /// Creates a new GltfBuffer with the specified byte length
    pub fn new(byte_length: u32) -> Self {
        GltfBuffer {
            uri: None,
            byte_length,
            name: None,
            extensions: None,
            extras: None,
        }
    }
}
