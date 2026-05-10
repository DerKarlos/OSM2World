use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// glTF Mesh
///
/// A set of primitives to be rendered as a single object.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct GltfMesh {
    pub primitives: Vec<Primitive>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weights: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

/// Mesh Primitive
///
/// Geometry to be rendered with the given material.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct Primitive {
    pub attributes: HashMap<String, u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indices: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

impl GltfMesh {
    /// Rendering mode: Points
    pub const POINTS: u32 = 0;
    /// Rendering mode: Lines
    pub const LINES: u32 = 1;
    /// Rendering mode: Line loop
    pub const LINE_LOOP: u32 = 2;
    /// Rendering mode: Line strip
    pub const LINE_STRIP: u32 = 3;
    /// Rendering mode: Triangles
    pub const TRIANGLES: u32 = 4;
    /// Rendering mode: Triangle strip
    pub const TRIANGLE_STRIP: u32 = 5;
    /// Rendering mode: Triangle fan
    pub const TRIANGLE_FAN: u32 = 6;

    /// Creates a new empty GltfMesh
    pub fn new() -> Self {
        GltfMesh {
            primitives: Vec::new(),
            weights: None,
            name: None,
            extensions: None,
            extras: None,
        }
    }
}

impl Default for GltfMesh {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for Primitive {
    fn default() -> Self {
        Primitive {
            attributes: HashMap::new(),
            indices: None,
            material: None,
            mode: None,
            extensions: None,
            extras: None,
        }
    }
}
