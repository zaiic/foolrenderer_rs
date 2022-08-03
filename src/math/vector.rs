use core::ops;
use crate::math::utility;

#[derive(Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, PartialEq)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

// One vector2 { 0., 0. }
#[macro_export]
macro_rules! vec2_zero {
    () => {
        Vec2 { x: 0., y: 0. }
    };
}

// One vector2 { 1., 1. }
#[macro_export]
macro_rules! vec2_one {
    () => {
        Vec2 { x: 1., y: 1. }
    };
}

// One vector3 { 0., 0., 0. }
#[macro_export]
macro_rules! vec3_zero {
    () => {
        Vec3 { x: 0., y: 0., z: 0. }
    };
}

// One vector3 { 1., 1., 1. }
#[macro_export]
macro_rules! vec3_one {
    () => {
        Vec3 { x: 1., y: 1., z: 1. }  
    };
}

// One vector4 { 0., 0., 0., 0. }
#[macro_export]
macro_rules! vec4_zero {
    () => {
        Vec4 { x: 0., y: 0., z: 0., w: 0. }
    };
}

// One vector4 { 1., 1., 1., 1. }
#[macro_export]
macro_rules! vec4_one {
    () => {
        Vec4 { x: 1., y: 1., z: 1., w: 1. }
    };
}

// ==================================================
// Vec2 implements
// ==================================================

impl Vec2 {
    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn into_vec3(self, z: f32) -> Vec3 {
        Vec3 { x: self.x, y: self.y, z }
    }
    
    // Dot multiply
    #[inline]
    pub fn dot(&self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    #[inline]
    pub fn magnitude(&self) -> f32 {
        self.dot(*self).sqrt()
    }

    #[inline]
    pub fn magnitude_squared(&self) -> f32 {
        self.dot(*self)
    }

    #[inline]
    pub fn normalize(self) -> Self {
        let squared_magnitude = self.magnitude_squared();
        if squared_magnitude == 0. {
            return vec2_zero!();
        } else if (squared_magnitude - 1.0).abs() < utility::SMALL_ABSOLUTE_F32 {
            return self;
        } else {
            self * (1. / squared_magnitude.sqrt())
        }
    }

    #[inline]
    pub fn lerp(&self, v: Self, t: f32) -> Self {
        Self {
            x: utility::f32_lerp(self.x, v.x, t),
            y: utility::f32_lerp(self.y, v.y, t),
        }
    }
}

impl From<&[f32; 2]> for Vec2 {
    #[inline]
    fn from(a: &[f32; 2]) -> Self {
        Self { x: a[0], y: a[1] }
    }
}
impl Clone for Vec2 {
    #[inline]
    fn clone(&self) -> Self {
        Self { x: self.x, y: self.y }
    }
}

impl Copy for Vec2 {}

impl ops::Add for Vec2 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl ops::Add<f32> for Vec2 {
    type Output = Self;
    
    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        Self { x: self.x + rhs, y: self.y + rhs }
    }
}

impl ops::Sub for Vec2 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl ops::Sub<f32> for Vec2 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        Self { x: self.x - rhs, y: self.y - rhs }
    }
}

impl ops::Mul for Vec2 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}

impl ops::Mul<f32> for Vec2 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}

impl ops::Div for Vec2 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self { x: self.x / rhs.x, y: self.y / rhs.y }
    }
}

impl ops::Div<f32> for Vec2 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Self { x: self.x / rhs, y: self.y / rhs }
    }
}

// ==================================================
// Vec3 implements
// ==================================================

impl Vec3 {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub fn into_vec2(self) -> Vec2 {
        Vec2 { x: self.x, y: self.y }
    }

    #[inline]
    pub fn into_vec4(self, w: f32) -> Vec4 {
        Vec4 { x: self.x, y: self.y, z: self.z, w }
    }

    // Dot multiply
    #[inline]
    pub fn dot(&self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    // Cross multiply
    #[inline]
    pub fn cross(&self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    #[inline]
    pub fn magnitude(&self) -> f32 {
        self.dot(*self).sqrt()
    }

    #[inline]
    pub fn magnitude_squared(&self) -> f32 {
        self.dot(*self)
    }

    #[inline]
    pub fn normalize(self) -> Self {
        let squared_magnitude = self.magnitude_squared();
        if squared_magnitude == 0. {
            return vec3_zero!();
        } else if (squared_magnitude - 1.0).abs() < utility::SMALL_ABSOLUTE_F32 {
            return self;
        } else {
            self * (1. / squared_magnitude.sqrt())
        }
    }

    #[inline]
    pub fn lerp(self, v: Self, t: f32) -> Self {
        Self {
            x: utility::f32_lerp(self.x, v.x, t),
            y: utility::f32_lerp(self.y, v.y, t),
            z: utility::f32_lerp(self.z, v.z, t)
        }
    }
}

impl From<&[f32; 3]> for Vec3 {
    #[inline]
    fn from(a: &[f32; 3]) -> Self {
        Self { x: a[0], y: a[1], z: a[2] }
    }
}

impl Clone for Vec3 {
    #[inline]
    fn clone(&self) -> Self {
        Self { x: self.x, y: self.y , z: self.z }
    }
}

impl Copy for Vec3 {}

impl ops::Add for Vec3 {
    type Output = Self;
    
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl ops::Add<f32> for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        Self { x: self.x + rhs, y: self.y + rhs, z: self.z + rhs }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}


impl ops::Sub<f32> for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        Self { x: self.x - rhs, y: self.y - rhs, z: self.z - rhs }
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl ops::Div for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self {x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Self {x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}

// ==================================================
// Vec4 implements
// ==================================================

impl Vec4 {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    #[inline]
    pub fn into_vec2(self) -> Vec2 {
        Vec2 { x: self.x, y: self.y }
    }

    #[inline]
    pub fn into_vec3(self) -> Vec3 {
        Vec3 { x: self.x, y: self.y, z: self.z }
    }

    #[inline]
    pub fn dot(&self, rhs: Self) -> f32 {
       self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w 
    }
    
    #[inline]
    pub fn magnitude(&self) -> f32 {
        self.dot(*self).sqrt()
    }

    #[inline]
    pub fn magnitude_squared(&self) -> f32 {
        self.dot(*self)
    }

    #[inline]
    pub fn normalize(self) -> Self {
        let squared_magnitude = self.magnitude_squared();
        if squared_magnitude == 0. {
            return vec4_zero!();
        } else if (squared_magnitude - 1.0).abs() < utility::SMALL_ABSOLUTE_F32 {
            return self;
        } else {
            self * (1. / squared_magnitude.sqrt())
        }
    }

    #[inline]
    pub fn lerp(&self, v: Self, t: f32) -> Self {
        Self {
            x: utility::f32_lerp(self.x, v.x, t),
            y: utility::f32_lerp(self.y, v.y, t),
            z: utility::f32_lerp(self.z, v.z, t),
            w: utility::f32_lerp(self.w, v.w, t),
        }
    }
}

impl From<&[f32; 4]> for Vec4 {
    #[inline]
    fn from(a: &[f32; 4]) -> Self {
        Self { x: a[0], y: a[1], z: a[2], w: a[3] }
    }
}

impl Clone for Vec4 {
    #[inline]
    fn clone(&self) -> Self {
        Self { x: self.x, y: self.y , z: self.z, w: self.w }
    }
}

impl Copy for Vec4 {}

impl ops::Add for Vec4 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z, w: self.w + rhs.w }
    }
}

impl ops::Add<f32> for Vec4 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        Self { x: self.x + rhs, y: self.y + rhs, z: self.z + rhs, w: self.w + rhs }
    }
}

impl ops::Sub for Vec4 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z, w: self.w - rhs.w }
    }
}

impl ops::Sub<f32> for Vec4 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        Self { x: self.x - rhs, y: self.y - rhs, z: self.z - rhs, w: self.w - rhs }
    }
}

impl ops::Mul for Vec4 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z, w: self.w * rhs.w }
    }
}

impl ops::Mul<f32> for Vec4 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs, w: self.w * rhs }
    }
}

impl ops::Div for Vec4 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self { x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z, w: self.w / rhs.w }
    }
}

impl ops::Div<f32> for Vec4 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Self { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs, w: self.w / rhs }
    }
}



