use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// glTF Accessor
/// 
/// Describes how to access an array of data in a buffer view.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct GltfAccessor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_view: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byte_offset: Option<u32>,
    pub component_type: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized: Option<bool>,
    pub count: u32,
    #[serde(rename = "type")]
    pub typ: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sparse: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

impl GltfAccessor {
    /// Component type: 8-bit signed integer
    pub const TYPE_BYTE: i32 = 5120;
    /// Component type: 8-bit unsigned integer
    pub const TYPE_UNSIGNED_BYTE: i32 = 5121;
    /// Component type: 16-bit signed integer
    pub const TYPE_SHORT: i32 = 5122;
    /// Component type: 16-bit unsigned integer
    pub const TYPE_UNSIGNED_SHORT: i32 = 5123;
    /// Component type: 32-bit unsigned integer
    pub const TYPE_UNSIGNED_INT: i32 = 5125;
    /// Component type: 32-bit floating point
    pub const TYPE_FLOAT: i32 = 5126;

    /// Creates a new GltfAccessor with validation
    pub fn new(component_type: i32, count: u32, typ: String) -> Result<Self, String> {
        if count == 0 {
            return Err(format!("invalid count: {}", count));
        }

        Ok(GltfAccessor {
            buffer_view: None,
            byte_offset: None,
            component_type,
            normalized: None,
            count,
            typ,
            max: None,
            min: None,
            sparse: None,
            name: None,
            extensions: None,
            extras: None,
        })
    }
}
