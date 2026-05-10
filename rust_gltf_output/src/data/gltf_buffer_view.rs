use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// glTF Buffer View
///
/// Describes a view into a buffer containing raw binary data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct GltfBufferView {
    pub buffer: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byte_offset: Option<u32>,
    pub byte_length: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byte_stride: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

impl GltfBufferView {
    /// Target: Array buffer (vertex attributes)
    pub const TARGET_ARRAY_BUFFER: u32 = 34962;
    /// Target: Element array buffer (indices)
    pub const TARGET_ELEMENT_ARRAY_BUFFER: u32 = 34963;

    /// Creates a new GltfBufferView
    pub fn new(buffer: u32, byte_length: u32) -> Self {
        GltfBufferView {
            buffer,
            byte_offset: None,
            byte_length,
            byte_stride: None,
            target: None,
            name: None,
            extensions: None,
            extras: None,
        }
    }
}
