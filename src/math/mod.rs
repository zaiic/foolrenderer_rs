pub mod utility;
pub mod vector;
pub mod matrix;

pub mod preclude {
    use super::*;
    pub use matrix::{ Mat3, Mat4 };
    pub use vector::{ Vec2, Vec3, Vec4 };
    pub use utility::*;
}

#[cfg(test)]
mod test;
