use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::transformation_matrix::TransformationMatrix;

/// glTF Node
///
/// A node in the node hierarchy. When the node contains skin, all
/// mesh.primitives MUST contain JOINTS_0 and WEIGHTS_0 attributes.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct GltfNode {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skin: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weights: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

impl GltfNode {
    /// Computes the local transformation matrix from TRS (translation, rotation, scale) properties
    pub fn get_local_transform(&self) -> Result<TransformationMatrix, String> {
        if let Some(matrix) = &self.matrix {
            if matrix.len() != 16 {
                return Err("Matrix must have 16 values".to_string());
            }
            let mut arr = [0.0f32; 16];
            for (i, v) in matrix.iter().enumerate() {
                arr[i] = *v;
            }
            TransformationMatrix::new(arr)
        } else {
            let translation = self.translation.clone().unwrap_or_else(|| vec![0.0, 0.0, 0.0]);
            let rotation = self.rotation.clone().unwrap_or_else(|| vec![0.0, 0.0, 0.0, 1.0]);
            let scale = self.scale.clone().unwrap_or_else(|| vec![1.0, 1.0, 1.0]);

            if translation.len() != 3 {
                return Err("Translation must have 3 values".to_string());
            }
            if rotation.len() != 4 {
                return Err("Rotation must have 4 values".to_string());
            }
            if scale.len() != 3 {
                return Err("Scale must have 3 values".to_string());
            }

            let translation_arr = [translation[0], translation[1], translation[2]];
            let rotation_arr = [rotation[0], rotation[1], rotation[2], rotation[3]];
            let scale_arr = [scale[0], scale[1], scale[2]];

            TransformationMatrix::for_trs(translation_arr, rotation_arr, scale_arr)
        }
    }
}

impl Default for GltfNode {
    fn default() -> Self {
        GltfNode {
            camera: None,
            children: None,
            skin: None,
            matrix: None,
            mesh: None,
            rotation: None,
            scale: None,
            translation: None,
            weights: None,
            name: None,
            extensions: None,
            extras: None,
        }
    }
}
