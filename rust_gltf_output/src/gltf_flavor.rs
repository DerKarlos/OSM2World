use std::str::FromStr;

/// Enumeration of glTF flavor types: JSON or binary format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GltfFlavor {
    /// glTF JSON format (.gltf)
    Gltf,
    /// glTF binary format (.glb)
    Glb,
}

impl GltfFlavor {
    /// Returns the file extension for this flavor
    pub fn extension(&self) -> String {
        match self {
            GltfFlavor::Gltf => ".gltf".to_string(),
            GltfFlavor::Glb => ".glb".to_string(),
        }
    }
}

impl FromStr for GltfFlavor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GLTF" => Ok(GltfFlavor::Gltf),
            "GLB" => Ok(GltfFlavor::Glb),
            _ => Err(format!("Invalid glTF flavor: {}", s)),
        }
    }
}

impl std::fmt::Display for GltfFlavor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GltfFlavor::Gltf => write!(f, "gltf"),
            GltfFlavor::Glb => write!(f, "glb"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extension() {
        assert_eq!(GltfFlavor::Gltf.extension(), ".gltf");
        assert_eq!(GltfFlavor::Glb.extension(), ".glb");
    }

    #[test]
    fn test_from_str() {
        assert_eq!("gltf".parse::<GltfFlavor>().unwrap(), GltfFlavor::Gltf);
        assert_eq!("glb".parse::<GltfFlavor>().unwrap(), GltfFlavor::Glb);
        assert_eq!("GLTF".parse::<GltfFlavor>().unwrap(), GltfFlavor::Gltf);
        assert!("invalid".parse::<GltfFlavor>().is_err());
    }

    #[test]
    fn test_display() {
        assert_eq!(GltfFlavor::Gltf.to_string(), "gltf");
        assert_eq!(GltfFlavor::Glb.to_string(), "glb");
    }
}
