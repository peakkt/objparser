use std::f32::consts::PI;

#[derive(Debug, Copy, Clone)]
pub struct Matrix4x4 {
    data: [[f32; 4]; 4],
}

#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Matrix4x4 {
    pub fn identity() -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn translation(tx: f32, ty: f32, tz: f32) -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, tx],
                [0.0, 1.0, 0.0, ty],
                [0.0, 0.0, 1.0, tz],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn scale(sx: f32, sy: f32, sz: f32) -> Self {
        Self {
            data: [
                [sx, 0.0, 0.0, 0.0],
                [0.0, sy, 0.0, 0.0],
                [0.0, 0.0, sz, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn rotation_x(theta: f32) -> Self {
        let radians = theta * PI / 180.0;
        let cos = radians.cos();
        let sin = radians.sin();
        
        Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, cos, -sin, 0.0],
                [0.0, sin, cos, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn rotation_y(theta: f32) -> Self {
        let radians = theta * PI / 180.0;
        let cos = radians.cos();
        let sin = radians.sin();

        Self {
            data: [
                [cos, 0.0, sin, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-sin, 0.0, cos, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn rotation_z(theta: f32) -> Self {
        let radians = theta * PI / 180.0;
        let cos = radians.cos();
        let sin = radians.sin();

        Self {
            data: [
                [cos, -sin, 0.0, 0.0],
                [sin, cos, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn multiply(&self, other: &Matrix4x4) -> Self {
        let mut result = Self::identity();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = 0.0;
                for k in 0..4 {
                    result.data[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        result
    }

    pub fn transform_vector(&self, vec: &Vector3) -> Vector3 {
        let x = vec.x * self.data[0][0] + vec.y * self.data[0][1] + vec.z * self.data[0][2] + self.data[0][3];
        let y = vec.x * self.data[1][0] + vec.y * self.data[1][1] + vec.z * self.data[1][2] + self.data[1][3];
        let z = vec.x * self.data[2][0] + vec.y * self.data[2][1] + vec.z * self.data[2][2] + self.data[2][3];
        Vector3 { x, y, z }
    }
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

