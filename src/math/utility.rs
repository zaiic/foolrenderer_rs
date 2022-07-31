pub const PI: f32 = 3.1415926535897932;
pub const SMALL_ABSOLUTE_F32: f32 = 0.000_000_01;

#[inline]
pub fn i32_max(a: i32, b: i32) -> i32 {
    match a > b {
        true => a,
        false => b,
    }
}

#[inline]
pub fn i32_min(a: i32, b: i32) -> i32 {
    match a > b {
        true => b,
        false => a,
    }
}

#[inline]
pub fn i32_clamp(n: i32, min: i32, max: i32) -> i32 {
    i32_min(i32_max(n, min), max)
}

#[inline]
pub fn u32_max(a: u32, b: u32) -> u32 {
    match a > b {
        true => a,
        false => b,
    }
}

#[inline]
pub fn u32_min(a: u32, b: u32) -> u32 {
    match a > b {
        true => b,
        false => a,
    }
}

#[inline]
pub fn u32_clamp(n: u32, min: u32, max: u32) -> u32 {
    u32_min(u32_max(n, min), max)
}

#[inline]
pub fn u32_clamp01(n: u32) -> u32 {
    u32_clamp(n, 0, 1)
}

#[inline]
pub fn f32_max(a: f32, b: f32) -> f32 {
    match a > b {
        true => a,
        false => b,
    }
}

#[inline]
pub fn f32_min(a: f32, b: f32) -> f32 {
    match a > b {
        true => b,
        false => a,
    }
}

#[inline]
pub fn f32_clamp(n: f32, min: f32, max: f32) -> f32 {
    f32_min(f32_max(n, min), max)
}

#[inline]
pub fn f32_clamp01(n: f32) -> f32 {
    f32_clamp(n, 0., 1.)
}

#[inline]
pub fn f32_lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}
