# OSM2World glTF Rust Module

This is a complete Rust transcoding of the OSM2World glTF output module from Java.

## Overview

This Rust module provides comprehensive support for reading and writing glTF (GL Transmission Format) and GLB (binary glTF) 3D model files. It maintains full compatibility with the original Java implementation while following Rust idioms and best practices.

## Features

- ✅ **Complete glTF 2.0 Support**: Full specification compliance
- ✅ **1:1 Class Mapping**: All Java classes transcoded to Rust structs
- ✅ **Preserved Naming**: All function and constant names kept identical
- ✅ **JSON Serialization**: Full serde support for JSON I/O
- ✅ **Transformation Matrices**: Complete matrix math utilities
- ✅ **Type Safety**: Leverages Rust's strong type system
- ✅ **Error Handling**: Comprehensive error handling with `Result` types

## Module Structure

```
src/
├── lib.rs                      # Library entry point
├── gltf_flavor.rs              # GltfFlavor enum
└── data/
    ├── mod.rs                  # Module exports
    ├── gltf.rs                 # Root Gltf structure
    ├── gltf_asset.rs           # Asset metadata
    ├── gltf_accessor.rs        # Data accessors
    ├── gltf_buffer.rs          # Buffer definitions
    ├── gltf_buffer_view.rs     # Buffer views
    ├── gltf_material.rs        # Material definitions
    ├── gltf_mesh.rs            # Mesh geometry
    ├── gltf_node.rs            # Scene graph nodes
    ├── gltf_sampler.rs         # Texture samplers
    ├── gltf_texture.rs         # Texture definitions
    ├── gltf_image.rs           # Image data
    ├── gltf_camera.rs          # Camera definitions
    ├── gltf_animation.rs       # Animation data
    ├── gltf_scene.rs           # Scene definitions
    ├── gltf_skin.rs            # Skeletal animation
    └── transformation_matrix.rs # 4x4 transformation matrices
```

## Dependencies

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## Usage Example

```rust
use osm2world_gltf_rust::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new glTF asset
    let mut gltf = Gltf::default();
    
    // Configure asset metadata
    gltf.asset.version = "2.0".to_string();
    gltf.asset.generator = "OSM2World".to_string();
    
    // Serialize to JSON
    let json = serde_json::to_string_pretty(&gltf)?;
    println!("{}", json);
    
    Ok(())
}
```

## Flavor Support

The module supports two glTF flavors:

- **GLTF**: JSON format (.gltf files)
- **GLB**: Binary format (.glb files)

```rust
use osm2world_gltf_rust::GltfFlavor;

let flavor = GltfFlavor::Gltf;
println!("Extension: {}", flavor.extension()); // Output: .gltf
```

## Data Structures

### Core Types

- `Gltf` - Root glTF asset structure
- `GltfAsset` - Asset metadata
- `GltfNode` - Scene graph node
- `GltfMesh` - Mesh geometry
- `GltfMaterial` - Material definition
- `GltfAccessor` - Typed data accessor
- `GltfBuffer` - Binary data container
- `GltfBufferView` - View into a buffer
- `TransformationMatrix` - 4x4 transformation matrix

### Material System

- `GltfMaterial` - Material properties
- `PbrMetallicRoughness` - PBR material model
- `TextureInfo` - Texture reference
- `NormalTextureInfo` - Normal map configuration
- `OcclusionTextureInfo` - Occlusion map configuration

## Transformation Matrices

The `TransformationMatrix` type provides full support for 4x4 transformation matrices used in scene graphs:

```rust
use osm2world_gltf_rust::data::TransformationMatrix;

// Create from components
let translate = [1.0, 2.0, 3.0];
let rotate = [0.0, 0.0, 0.7071, 0.7071]; // quaternion
let scale = [1.0, 1.0, 1.0];

let matrix = TransformationMatrix::for_trs(translate, rotate, scale)?;

// Multiply matrices
let result = matrix.times(&other_matrix);

// Transpose
let transposed = matrix.transpose()?;
```

## Serialization

All structures support JSON serialization via serde:

```rust
let gltf = Gltf::default();

// To JSON
let json = serde_json::to_string(&gltf)?;

// From JSON
let parsed: Gltf = serde_json::from_str(&json)?;
```

## glTF 2.0 Specification

For complete documentation on glTF 2.0, refer to:
https://github.com/KhronosGroup/glTF/blob/master/specification/2.0/README.md

## License

This transcoding maintains the same license as the original OSM2World project.

## Contributing

Contributions are welcome! Please ensure:

1. All tests pass: `cargo test`
2. Code follows Rust conventions: `cargo fmt && cargo clippy`
3. 1:1 compatibility with Java implementation is maintained
