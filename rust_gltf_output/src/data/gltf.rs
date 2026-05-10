use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::*;

/// A glTF asset - a simple mutable struct holding all data.
///
/// Refer to the spec at https://github.com/KhronosGroup/glTF/blob/master/specification/2.0/README.md
/// for comprehensive documentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct Gltf {
    pub asset: GltfAsset,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions_used: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions_required: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessors: Option<Vec<GltfAccessor>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animations: Option<Vec<GltfAnimation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffers: Option<Vec<GltfBuffer>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_views: Option<Vec<GltfBufferView>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cameras: Option<Vec<GltfCamera>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<GltfImage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub materials: Option<Vec<GltfMaterial>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meshes: Option<Vec<GltfMesh>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<GltfNode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub samplers: Option<Vec<GltfSampler>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenes: Option<Vec<GltfScene>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skins: Option<Vec<GltfSkin>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub textures: Option<Vec<GltfTexture>>,
}

impl Default for Gltf {
    fn default() -> Self {
        Gltf {
            asset: GltfAsset::default(),
            extensions_used: None,
            extensions_required: None,
            accessors: None,
            animations: None,
            buffers: None,
            buffer_views: None,
            cameras: None,
            images: None,
            materials: None,
            meshes: None,
            nodes: None,
            samplers: None,
            scene: None,
            scenes: None,
            skins: None,
            textures: None,
        }
    }
}
