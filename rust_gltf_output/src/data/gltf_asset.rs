use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// glTF Asset Information
/// 
/// Describes metadata about the glTF asset, including version and generator information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct GltfAsset {
    /// The glTF version number (typically "2.0")
    pub version: String,
    /// The name and version of the tool that created this asset
    pub generator: String,
}

impl Default for GltfAsset {
    fn default() -> Self {
        GltfAsset {
            version: String::new(),
            generator: String::new(),
        }
    }
}
