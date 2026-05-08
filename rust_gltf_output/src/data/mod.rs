//! glTF data structures module
//! 
//! This module contains all the data structures used to represent
//! glTF assets according to the glTF 2.0 specification.

pub mod gltf;
pub mod gltf_accessor;
pub mod gltf_animation;
pub mod gltf_asset;
pub mod gltf_buffer;
pub mod gltf_buffer_view;
pub mod gltf_camera;
pub mod gltf_image;
pub mod gltf_material;
pub mod gltf_mesh;
pub mod gltf_node;
pub mod gltf_sampler;
pub mod gltf_scene;
pub mod gltf_skin;
pub mod gltf_texture;
pub mod transformation_matrix;

pub use gltf::Gltf;
pub use gltf_accessor::GltfAccessor;
pub use gltf_animation::GltfAnimation;
pub use gltf_asset::GltfAsset;
pub use gltf_buffer::GltfBuffer;
pub use gltf_buffer_view::GltfBufferView;
pub use gltf_camera::GltfCamera;
pub use gltf_image::GltfImage;
pub use gltf_material::{GltfMaterial, TextureInfo, NormalTextureInfo, OcclusionTextureInfo, PbrMetallicRoughness};
pub use gltf_mesh::{GltfMesh, Primitive};
pub use gltf_node::GltfNode;
pub use gltf_sampler::GltfSampler;
pub use gltf_scene::GltfScene;
pub use gltf_skin::GltfSkin;
pub use gltf_texture::GltfTexture;
pub use transformation_matrix::TransformationMatrix;
