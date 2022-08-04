use super::preclude::*;
use std::ops;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Mat3 {
    pub elem: [[f32; 3]; 3],        // 3x3 matrix
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Mat4 {
    pub elem: [[f32; 4]; 4],        // 4x4 matrix
}

#[macro_export]
macro_rules! mat3_identity {
    () => {
        Mat3 {elem: [
            [1., 0., 0.],
            [0., 1., 0.],
            [0., 0., 1.],
        ]}
    };
}

#[macro_export]
macro_rules! mat3_zero {
    () => {
        Mat3 {elem: [
            [0., 0., 0.],
            [0., 0., 0.],
            [0., 0., 0.],
        ]}
    };
}

#[macro_export]
macro_rules! mat4_identity {
    () => {
        Mat4 {elem: [
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ]}
    };
}

#[macro_export]
macro_rules! mat4_zero {
    () => {
        Mat4 {elem: [
            [0., 0., 0., 0.],
            [0., 0., 0., 0.],
            [0., 0., 0., 0.],
            [0., 0., 0., 0.],
        ]}
    };
}

// ==================================================
// Mat3 implements
// ==================================================

impl Mat3 {
    #[inline]
    pub fn new() -> Self {
        mat3_zero!()
    }

    #[inline]
    pub fn from_vec3(v1: Vec3, v2: Vec3, v3: Vec3) -> Self {
        Self { elem:[
            [v1.x, v2.x, v3.x],
            [v1.y, v2.y, v3.y],
            [v1.z, v2.z, v3.z],
        ]}
    }

    #[inline]
    pub fn transpose(self) -> Self {
        let mut m = Self::new();
        for i in 0..3 {
            for j in 0..3 {
                m.elem[i][j] = self.elem[j][i];
            }
        }

        m
    }
}

impl Display for Mat3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[[{}, {}, {}],\n [{}, {}, {}],\n [{}, {}, {}]]",
               self.elem[0][0], self.elem[0][1], self.elem[0][2],
               self.elem[1][0], self.elem[1][1], self.elem[1][2],
               self.elem[2][0], self.elem[2][1], self.elem[2][2],
               )
    }
}

impl ops::Mul<f32> for Mat3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        let mut m = Self::new();
        for i in 0..3 {
            for j in 0..3 {
                m.elem[i][j] = self.elem[i][j] * rhs;
            }
        }

        m
    }
}

impl ops::Mul<Vec3> for Mat3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        let mut v = Vec3::from(&[0., 0., 0.]);
        v.x = self.elem[0][0] * rhs.x + self.elem[0][1] * rhs.y + self.elem[0][2] * rhs.z;
        v.y = self.elem[1][0] * rhs.x + self.elem[1][1] * rhs.y + self.elem[1][2] * rhs.z;
        v.z = self.elem[2][0] * rhs.x + self.elem[2][1] * rhs.y + self.elem[2][2] * rhs.z;

        v
    }
}

impl ops::Mul for Mat3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let mut m = Self::new();
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    m.elem[i][j] += self.elem[i][k] * rhs.elem[k][j];
                }
            }
        }

        m
    }
}

impl ops::Index<usize> for Mat3 {
    type Output = [f32; 3];

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.elem[index]
    }
}

impl ops::IndexMut<usize> for Mat3 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elem[index]
    }
}

// ==================================================
// Mat4 implements
// ==================================================

impl Mat4 {
    #[inline]
    pub fn new() -> Self {
        mat4_zero!()
    }

    #[inline]
    pub fn from_vec4(v1: Vec4, v2: Vec4, v3: Vec4, v4: Vec4) -> Self {
        Self { elem:[
            [v1.x, v2.x, v3.x, v4.x],
            [v1.y, v2.y, v3.y, v4.y],
            [v1.z, v2.z, v3.z, v4.z],
            [v1.w, v2.w, v3.w, v4.w],
        ]}
    }

    #[inline]
    pub fn scale(scaling: Vec3) -> Self {
        let mut m = mat4_identity!();
        m.elem[0][0] = scaling.x;
        m.elem[1][1] = scaling.y;
        m.elem[2][2] = scaling.z;
        
        m
    }

    #[inline]
    pub fn translate(translation: Vec3) -> Self {
        let mut m = mat4_identity!();
        m.elem[0][3] = translation.x;
        m.elem[1][3] = translation.y;
        m.elem[2][3] = translation.z;

        m
    }

    #[inline]
    pub fn rotate_x(angle: f32) -> Self {
        let (s, c) = angle.sin_cos();
        let mut m = mat4_identity!();
        m.elem[1][1] = c;
        m.elem[1][2] = -s;
        m.elem[2][1] = s;
        m.elem[2][2] = c;

        m
    }

    #[inline]
    pub fn rotate_y(angle: f32) -> Self {
        let (s, c) = angle.sin_cos();
        let mut m = mat4_identity!();
        m.elem[0][0] = c;
        m.elem[0][2] = s;
        m.elem[2][0] = -s;
        m.elem[2][2] = c;

        m
    }

    #[inline]
    pub fn rotate_z(angle: f32) -> Self {
        let (s, c) = angle.sin_cos();
        let mut m = mat4_identity!();
        m.elem[0][0] = c;
        m.elem[0][1] = -s;
        m.elem[1][0] = s;
        m.elem[1][1] = c;

        m
    }

    pub fn rotate_about(angle: f32, about: Vec3) -> Self {
        if about.x == 1. && about.y == 0. && about.z == 0. {
           return Self::rotate_x(angle);
        }
        if about.x == 0. && about.y == 1. && about.z == 0. {
           return Self::rotate_y(angle);
        }
        if about.x == 0. && about.y == 0. && about.z == 1. {
            return Self::rotate_z(angle);
        }

        let mut m = mat4_identity!();
        let (s, c) = angle.sin_cos();
        let about = about.normalize();
        let nc = 1. - c;
        let (xy, yz, zx, xs, ys, zs) = (about.x * about.y, about.y * about.z, about.z * about.x,
                                        about.x * s, about.y * s, about.z * s);

        m.elem[0][0] = about.x * about.x * nc + c;
        m.elem[0][1] = xy * nc - zs;
        m.elem[0][2] = zx * nc + ys;

        m.elem[1][0] = xy * nc + zs;
        m.elem[1][1] = about.y * about.y * nc + c;
        m.elem[1][2] = yz * nc - xs;

        m.elem[2][0] = zx * nc - ys;
        m.elem[2][1] = yz * nc + xs;
        m.elem[2][2] = about.z * about.z * nc + c;

        m
    }

    #[inline]
    pub fn look_at(from: Vec3, to: Vec3, up: Vec3) -> Self {
        let z_axis = (from - to).normalize();
        let x_axis = up.cross(z_axis).normalize();
        let y_axis = z_axis.cross(x_axis);
        let mut m = mat4_identity!();

        m.elem[0][0] = x_axis.x;
        m.elem[0][1] = x_axis.y;
        m.elem[0][2] = x_axis.z;

        m.elem[1][0] = y_axis.x;
        m.elem[1][1] = y_axis.y;
        m.elem[1][2] = y_axis.z;

        m.elem[2][0] = z_axis.x;
        m.elem[2][1] = z_axis.y;
        m.elem[2][2] = z_axis.z;

        m.elem[0][3] = -(x_axis.dot(from));
        m.elem[1][3] = -(y_axis.dot(from));
        m.elem[2][3] = -(z_axis.dot(from));

        m
    }

    #[inline]
    pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        let mut m = mat4_zero!();
        let distance = far - near;
        m.elem[1][1] = 1./ ((fov / 2.).tan());
        m.elem[0][0] = m.elem[1][1] / aspect;
        m.elem[2][2] = (-far - near) / distance;
        m.elem[2][3] = (-2. * far * near) / distance;
        m.elem[3][2] = -1.;

        m
    }

    #[inline]
    pub fn orthographic(right: f32, top: f32, near: f32, far: f32) -> Self {
        let mut m = mat4_identity!();
        let distance = far - near;
        m.elem[0][0] = 1. / right;
        m.elem[1][1] = 1. / top;
        m.elem[2][2] = -2. / distance;
        m.elem[2][3] = (-near - far) / distance;

        m
    }

    #[inline]
    pub fn into_mat3(self) -> Mat3 {
        Mat3 {elem: [
            [self.elem[0][0], self.elem[0][1], self.elem[0][2]],
            [self.elem[1][0], self.elem[1][1], self.elem[1][2]],
            [self.elem[2][0], self.elem[2][1], self.elem[2][2]],
        ]}
    }

    #[inline]
    pub fn transpose(self) -> Self {
        let mut m = Self::new();
        for i in 0..4 {
            for j in 0..4 {
                m.elem[i][j] = self.elem[j][i];
            }
        }

        m
    }
}

impl ops::Mul<f32> for Mat4 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        let mut m = Self::new();
        for i in 0..4 {
            for j in 0..4 {
                m.elem[i][j] = self.elem[i][j] * rhs;
            }
        }

        m
    }
}

impl ops::Mul<Vec4> for Mat4 {
    type Output = Vec4;

    #[inline]
    fn mul(self, rhs: Vec4) -> Self::Output {
        let mut v = Vec4::from(&[0., 0., 0., 0.]);
        v.x = self.elem[0][0] * rhs.x + self.elem[0][1] * rhs.y + self.elem[0][2] * rhs.z + self.elem[0][3] * rhs.w;
        v.y = self.elem[1][0] * rhs.x + self.elem[1][1] * rhs.y + self.elem[1][2] * rhs.z + self.elem[1][3] * rhs.w;
        v.z = self.elem[2][0] * rhs.x + self.elem[2][1] * rhs.y + self.elem[2][2] * rhs.z + self.elem[2][3] * rhs.w;
        v.w = self.elem[3][0] * rhs.x + self.elem[3][1] * rhs.y + self.elem[3][2] * rhs.z + self.elem[3][3] * rhs.w;

        v
    }
}


impl ops::Mul for Mat4 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let mut m = Self::new();
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    m.elem[i][j] += self.elem[i][k] * rhs.elem[k][j];
                }
            }
        }

        m
    }
}

impl ops::Not for Mat4 {
    type Output = Self;

    fn not(self) -> Self::Output {
        let (a11, a12, a13, a14) = (self.elem[0][0], self.elem[0][1], self.elem[0][2], self.elem[0][3]);
        let (a21, a22, a23, a24) = (self.elem[1][0], self.elem[1][1], self.elem[1][2], self.elem[1][3]);
        let (a31, a32, a33, a34) = (self.elem[2][0], self.elem[2][1], self.elem[2][2], self.elem[2][3]);
        let (a41, a42, a43, a44) = (self.elem[3][0], self.elem[3][1], self.elem[3][2], self.elem[3][3]);

        // Use the adjugate of the matrix to calculates the inverse.
        let mut adj = Mat4::new();
        adj.elem[0][0] = a22 * a33 * a44 + a23 * a34 * a42 + a24 * a32 * a43 -
                         a24 * a33 * a42 - a23 * a32 * a44 - a22 * a34 * a43;
        adj.elem[0][1] = -a12 * a33 * a44 - a13 * a34 * a42 - a14 * a32 * a43 +
                         a14 * a33 * a42 + a13 * a32 * a44 + a12 * a34 * a43;
        adj.elem[0][2] = a12 * a23 * a44 + a13 * a24 * a42 + a14 * a22 * a43 -
                         a14 * a23 * a42 - a13 * a22 * a44 - a12 * a24 * a43;
        adj.elem[0][3] = -a12 * a23 * a34 - a13 * a24 * a32 - a14 * a22 * a33 +
                         a14 * a23 * a32 + a13 * a22 * a34 + a12 * a24 * a33;

        adj.elem[1][0] = -a21 * a33 * a44 - a23 * a34 * a41 - a24 * a31 * a43 +
                         a24 * a33 * a41 + a23 * a31 * a44 + a21 * a34 * a43;
        adj.elem[1][1] = a11 * a33 * a44 + a13 * a34 * a41 + a14 * a31 * a43 -
                         a14 * a33 * a41 - a13 * a31 * a44 - a11 * a34 * a43;
        adj.elem[1][2] = -a11 * a23 * a44 - a13 * a24 * a41 - a14 * a21 * a43 +
                         a14 * a23 * a41 + a13 * a21 * a44 + a11 * a24 * a43;
        adj.elem[1][3] = a11 * a23 * a34 + a13 * a24 * a31 + a14 * a21 * a33 -
                         a14 * a23 * a31 - a13 * a21 * a34 - a11 * a24 * a33;

        adj.elem[2][0] = a21 * a32 * a44 + a22 * a34 * a41 + a24 * a31 * a42 -
                         a24 * a32 * a41 - a22 * a31 * a44 - a21 * a34 * a42;
        adj.elem[2][1] = -a11 * a32 * a44 - a12 * a34 * a41 - a14 * a31 * a42 +
                         a14 * a32 * a41 + a12 * a31 * a44 + a11 * a34 * a42;
        adj.elem[2][2] = a11 * a22 * a44 + a12 * a24 * a41 + a14 * a21 * a42 -
                         a14 * a22 * a41 - a12 * a21 * a44 - a11 * a24 * a42;
        adj.elem[2][3] = -a11 * a22 * a34 - a12 * a24 * a31 - a14 * a21 * a32 +
                         a14 * a22 * a31 + a12 * a21 * a34 + a11 * a24 * a32;

        adj.elem[3][0] = -a21 * a32 * a43 - a22 * a33 * a41 - a23 * a31 * a42 +
                         a23 * a32 * a41 + a22 * a31 * a43 + a21 * a33 * a42;
        adj.elem[3][1] = a11 * a32 * a43 + a12 * a33 * a41 + a13 * a31 * a42 -
                         a13 * a32 * a41 - a12 * a31 * a43 - a11 * a33 * a42;
        adj.elem[3][2] = -a11 * a22 * a43 - a12 * a23 * a41 - a13 * a21 * a42 +
                         a13 * a22 * a41 + a12 * a21 * a43 + a11 * a23 * a42;
        adj.elem[3][3] = a11 * a22 * a33 + a12 * a23 * a31 + a13 * a21 * a32 -
                         a13 * a22 * a31 - a12 * a21 * a33 - a11 * a23 * a32;

        let determinant = a11 * adj.elem[0][0] + a21 * adj.elem[0][1] +
                        a31 * adj.elem[0][2] + a41 * adj.elem[0][3];

        if determinant == 0. {
            // The matrix is not invertible.
            return mat4_zero!()
        }

        adj * (1. / determinant)
    }
}

impl ops::Index<usize> for Mat4 {
    type Output = [f32; 4];

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.elem[index]
    }
}

impl ops::IndexMut<usize> for Mat4 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elem[index]
    }
}

impl Display for Mat4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[[{}, {}, {}, {}],\n [{}, {}, {}, {}],\n [{}, {}, {}, {}],\n [{}, {}, {}, {}]]",
               self.elem[0][0], self.elem[0][1], self.elem[0][2], self.elem[0][3],
               self.elem[1][0], self.elem[1][1], self.elem[1][2], self.elem[1][3],
               self.elem[2][0], self.elem[2][1], self.elem[2][2], self.elem[2][3],
               self.elem[3][0], self.elem[3][1], self.elem[3][2], self.elem[3][3],
               )
    }
}








