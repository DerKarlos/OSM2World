use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// glTF Sampler
///
/// Texture sampler properties for filtering and wrapping.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct GltfSampler {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_filter: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mag_filter: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrap_s: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrap_t: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

impl GltfSampler {
    // Wrapping modes
    /// Wrapping mode: Clamp to edge
    pub const WRAP_CLAMP_TO_EDGE: u32 = 33071;
    /// Wrapping mode: Mirrored repeat
    pub const WRAP_MIRRORED_REPEAT: u32 = 33648;
    /// Wrapping mode: Repeat
    pub const WRAP_REPEAT: u32 = 10497;

    // Filtering modes
    /// Filter: Nearest
    pub const NEAREST: u32 = 9728;
    /// Filter: Linear
    pub const LINEAR: u32 = 9729;
    /// Filter: Nearest mipmap nearest
    pub const NEAREST_MIPMAP_NEAREST: u32 = 9984;
    /// Filter: Linear mipmap nearest
    pub const LINEAR_MIPMAP_NEAREST: u32 = 9985;
    /// Filter: Nearest mipmap linear
    pub const NEAREST_MIPMAP_LINEAR: u32 = 9986;
    /// Filter: Linear mipmap linear
    pub const LINEAR_MIPMAP_LINEAR: u32 = 9987;
}

impl Default for GltfSampler {
    fn default() -> Self {
        GltfSampler {
            min_filter: None,
            mag_filter: None,
            wrap_s: None,
            wrap_t: None,
            name: None,
            extensions: None,
            extras: None,
        }
    }
}
