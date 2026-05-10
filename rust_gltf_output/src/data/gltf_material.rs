use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Texture Information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct TextureInfo {
    pub index: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tex_coord: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

/// Normal Texture Information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct NormalTextureInfo {
    pub index: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tex_coord: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

/// Occlusion Texture Information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct OcclusionTextureInfo {
    pub index: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tex_coord: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

/// PBR Metallic Roughness
///
/// Defines the metallic/roughness material model.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct PbrMetallicRoughness {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_color_factor: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_color_texture: Option<TextureInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metallic_factor: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roughness_factor: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metallic_roughness_texture: Option<TextureInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

/// glTF Material
///
/// Describes the appearance of geometry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct GltfMaterial {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pbr_metallic_roughness: Option<PbrMetallicRoughness>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal_texture: Option<NormalTextureInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occlusion_texture: Option<OcclusionTextureInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emissive_texture: Option<TextureInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emissive_factor: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha_cutoff: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_sided: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

impl Default for GltfMaterial {
    fn default() -> Self {
        GltfMaterial {
            pbr_metallic_roughness: None,
            normal_texture: None,
            occlusion_texture: None,
            emissive_texture: None,
            emissive_factor: None,
            alpha_mode: None,
            alpha_cutoff: None,
            double_sided: None,
            name: None,
            extensions: None,
            extras: None,
        }
    }
}
