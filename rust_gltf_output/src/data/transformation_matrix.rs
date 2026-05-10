/// 4x4 transformation matrix
///
/// Represents a 4x4 transformation matrix with values stored in column-major order.
/// Used for node transformations in glTF hierarchies.
#[derive(Debug, Clone)]
pub struct TransformationMatrix {
    /// 16 float values in column-major order
    pub values: [f32; 16],
}

impl TransformationMatrix {
    /// Creates a new transformation matrix from 16 values
    pub fn new(values: [f32; 16]) -> Result<Self, String> {
        Ok(TransformationMatrix { values })
    }

    /// Gets a row of the matrix
    pub fn row(&self, i: usize) -> Result<[f32; 4], String> {
        if i > 3 {
            return Err(format!("illegal row index: {}", i));
        }
        Ok([
            self.values[i * 4],
            self.values[i * 4 + 1],
            self.values[i * 4 + 2],
            self.values[i * 4 + 3],
        ])
    }

    /// Gets a column of the matrix
    pub fn col(&self, i: usize) -> Result<[f32; 4], String> {
        if i > 3 {
            return Err(format!("illegal column index: {}", i));
        }
        Ok([
            self.values[i],
            self.values[i + 4],
            self.values[i + 8],
            self.values[i + 12],
        ])
    }

    /// Gets a specific element at (col, row)
    pub fn get(&self, col: usize, row: usize) -> f32 {
        self.values[col * 4 + row]
    }

    /// Returns the transpose of this matrix
    pub fn transpose(&self) -> Result<TransformationMatrix, String> {
        let mut result = [0.0f32; 16];
        for i in 0..4 {
            for j in 0..4 {
                result[i * 4 + j] = self.get(j, i);
            }
        }
        TransformationMatrix::new(result)
    }

    /// Multiplies this matrix with another matrix
    pub fn times(&self, m: &TransformationMatrix) -> TransformationMatrix {
        let mut result = [0.0f32; 16];
        for row in 0..4 {
            for col in 0..4 {
                for k in 0..4 {
                    result[col * 4 + row] += self.get(k, row) * m.get(col, k);
                }
            }
        }
        TransformationMatrix {
            values: result,
        }
    }

    /// Creates a translation matrix
    pub fn for_translation(translation: [f32; 3]) -> Result<TransformationMatrix, String> {
        let values = [
            1.0, 0.0, 0.0, translation[0],
            0.0, 1.0, 0.0, translation[1],
            0.0, 0.0, 1.0, translation[2],
            0.0, 0.0, 0.0, 1.0,
        ];
        let mut m = TransformationMatrix::new(values)?;
        m = m.transpose()?;
        Ok(m)
    }

    /// Creates a rotation matrix from a quaternion [x, y, z, w]
    pub fn for_rotation(rotation: [f32; 4]) -> Result<TransformationMatrix, String> {
        let mut qx = rotation[0];
        let mut qy = rotation[1];
        let mut qz = rotation[2];
        let mut qw = rotation[3];

        // Normalize the quaternion
        let norm = (qx * qx + qy * qy + qz * qz + qw * qw).sqrt();
        qx /= norm;
        qy /= norm;
        qz /= norm;
        qw /= norm;

        let values = [
            1.0 - 2.0 * qy * qy - 2.0 * qz * qz, 2.0 * qx * qy - 2.0 * qz * qw, 2.0 * qx * qz + 2.0 * qy * qw, 0.0,
            2.0 * qx * qy + 2.0 * qz * qw, 1.0 - 2.0 * qx * qx - 2.0 * qz * qz, 2.0 * qy * qz - 2.0 * qx * qw, 0.0,
            2.0 * qx * qz - 2.0 * qy * qw, 2.0 * qy * qz + 2.0 * qx * qw, 1.0 - 2.0 * qx * qx - 2.0 * qy * qy, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ];
        let mut m = TransformationMatrix::new(values)?;
        m = m.transpose()?;
        Ok(m)
    }

    /// Creates a scale matrix
    pub fn for_scale(scale: [f32; 3]) -> Result<TransformationMatrix, String> {
        let values = [
            scale[0], 0.0, 0.0, 0.0,
            0.0, scale[1], 0.0, 0.0,
            0.0, 0.0, scale[2], 0.0,
            0.0, 0.0, 0.0, 1.0,
        ];
        TransformationMatrix::new(values)
    }

    /// Creates a composite transformation matrix from translation, rotation, and scale
    pub fn for_trs(translation: [f32; 3], rotation: [f32; 4], scale: [f32; 3]) -> Result<TransformationMatrix, String> {
        let t = Self::for_translation(translation)?;
        let r = Self::for_rotation(rotation)?;
        let s = Self::for_scale(scale)?;
        Ok(t.times(&r).times(&s))
    }

    /// Computes the transposed inverse of the 3x3 portion (for normal transformation)
    fn transposed_inverse_3x3(&self) -> Result<[[f32; 3]; 3], String> {
        let det = self.get(0, 0) * (self.get(1, 1) * self.get(2, 2) - self.get(2, 1) * self.get(1, 2))
            - self.get(0, 1) * (self.get(1, 0) * self.get(2, 2) - self.get(1, 2) * self.get(2, 0))
            + self.get(0, 2) * (self.get(1, 0) * self.get(2, 1) - self.get(1, 1) * self.get(2, 0));

        if det.abs() < 1e-10 {
            return Err("Matrix is singular".to_string());
        }

        let invdet = 1.0 / det;

        let mut result = [[0.0f32; 3]; 3];

        result[0][0] = (self.get(1, 1) * self.get(2, 2) - self.get(2, 1) * self.get(1, 2)) * invdet;
        result[1][0] = -(self.get(0, 1) * self.get(2, 2) - self.get(0, 2) * self.get(2, 1)) * invdet;
        result[2][0] = (self.get(0, 1) * self.get(1, 2) - self.get(0, 2) * self.get(1, 1)) * invdet;
        result[0][1] = -(self.get(1, 0) * self.get(2, 2) - self.get(1, 2) * self.get(2, 0)) * invdet;
        result[1][1] = (self.get(0, 0) * self.get(2, 2) - self.get(0, 2) * self.get(2, 0)) * invdet;
        result[2][1] = -(self.get(0, 0) * self.get(1, 2) - self.get(1, 0) * self.get(0, 2)) * invdet;
        result[0][2] = (self.get(1, 0) * self.get(2, 1) - self.get(2, 0) * self.get(1, 1)) * invdet;
        result[1][2] = -(self.get(0, 0) * self.get(2, 1) - self.get(2, 0) * self.get(0, 1)) * invdet;
        result[2][2] = (self.get(0, 0) * self.get(1, 1) - self.get(1, 0) * self.get(0, 1)) * invdet;

        Ok(result)
    }
}

impl Default for TransformationMatrix {
    fn default() -> Self {
        // Identity matrix
        TransformationMatrix {
            values: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }
}

impl std::cmp::PartialEq for TransformationMatrix {
    fn eq(&self, other: &Self) -> bool {
        self.values.iter().zip(other.values.iter()).all(|(a, b)| (a - b).abs() < 1e-10)
    }
}

impl std::fmt::Display for TransformationMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self.values[..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_matrix() {
        let m = TransformationMatrix::default();
        assert_eq!(m.get(0, 0), 1.0);
        assert_eq!(m.get(1, 1), 1.0);
        assert_eq!(m.get(2, 2), 1.0);
        assert_eq!(m.get(3, 3), 1.0);
    }

    #[test]
    fn test_transpose() {
        let values = [
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 10.0, 11.0, 12.0,
            13.0, 14.0, 15.0, 16.0,
        ];
        let m = TransformationMatrix::new(values).unwrap();
        let t = m.transpose().unwrap();
        assert_eq!(t.get(1, 0), m.get(0, 1));
    }
}
