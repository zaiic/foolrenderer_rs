const GAMMA: f32 = 2.2;


#[inline]
pub fn f32_to_u8(value: f32) -> u8 {
    (value * 0xff as f32) as u8
}

#[inline]
pub fn u8_to_f32(value: u8) -> f32 {
    value as f32 / 255.
}

#[inline]
pub fn convert_to_srgb_color(value: f32) -> f32 {
    value.powf(1. / GAMMA)
}

#[inline]
pub fn convert_to_linear_color(value: f32) -> f32 {
    value.powf(GAMMA)
}
