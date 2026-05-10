//! Rust transcoding of OSM2World glTF output module
//!
//! This module provides glTF and GLB format output support for 3D models.
//! It maintains a 1:1 mapping with the original Java implementation while
//! following Rust idioms and best practices.

pub mod data;
pub mod gltf_flavor;

pub use data::*;
pub use gltf_flavor::GltfFlavor;
