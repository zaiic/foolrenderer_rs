use std::convert::TryInto;

use crate::math::preclude::{Vec2, Vec4};
use crate::math::utility::*;
use crate::vec4_one;

use super::color::{u8_to_f32, convert_to_linear_color};

#[derive(PartialEq, Eq,Debug, Clone)]
pub enum TextureFormat {
    R8,
    RGB8,
    SRGB8,
    RGBA8,
    SRGB8_A8,
    DEPTH_FLOAT,
}

#[derive(Debug)]
pub struct Texture {
    pub format: TextureFormat,
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
}

impl Texture {
    pub fn new(format: TextureFormat, width: u32, height: u32) -> Option<Self> {
        if width == 0 || height == 0 {
            return None
        }

        let pixel_size = match format {
            TextureFormat::R8 => 1,
            TextureFormat::RGB8 | TextureFormat::SRGB8 => 3,
            TextureFormat::RGBA8 | TextureFormat::SRGB8_A8 => 4,
            TextureFormat::DEPTH_FLOAT => 4,
        };

        Some(Self {
            format,
            width,
            height,
            pixels: vec![255; (width * height * pixel_size) as usize],
        })
    }

    pub fn get_texture_format(&self) -> TextureFormat {
        self.format.clone()
    }

    pub fn set_texture_pixels(&mut self, pixels: &[u8]) -> Result<(), &str>{
        let pixel_size = self.get_pixel_size();

        match (self.height * self.width * pixel_size as u32) as usize >= pixels.len() {
            true => {
                // TODO: Optimize the efficient and decline RAM pressure.
                for current_pixel in self.pixels.chunks_exact_mut(pixels.len()) {
                    current_pixel.copy_from_slice(pixels);
                }

                Ok(())
            },
            false => {
                Err("[Error] Set texture failed.")
            }
        }
    }

    pub fn get_texture_pixels(&mut self) -> &mut Vec<u8> {
        &mut self.pixels
    }

    #[inline]
    pub fn get_pixel_size(&self) -> i32 {
        let pixel_size = match self.format {
            TextureFormat::R8 => 1,
            TextureFormat::RGB8 | TextureFormat::SRGB8 => 3,
            TextureFormat::RGBA8 | TextureFormat::SRGB8_A8 => 4,
            TextureFormat::DEPTH_FLOAT => 4,
        };
        pixel_size
    }

    pub fn get_shape(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn texture_sample(&self, texcoord: Vec2) -> Vec4 {
        let u = f32_clamp01(texcoord.x);
        let v = f32_clamp01(texcoord.y);
        let mut u_index = (u * self.width as f32) as u32;
        let mut v_index = (v * self.height as f32) as u32;
        if u_index >= self.width { u_index = self.width - 1 }
        if v_index >= self.height { v_index = self.height - 1 }
        let pixel_offset: usize = (u_index + v_index * self.width).try_into().unwrap();
        let pixel_size = self.get_pixel_size();
        /*
         * pixel.x: red
         * pixel.y: green
         * pixel.z: blue
         */
        let mut pixel = vec4_one!();
        match self.format {
            TextureFormat::DEPTH_FLOAT => {
                let target = self.pixels[pixel_offset];
                pixel.x = target.into();
                pixel.y = target.into();
                pixel.z = target.into();
           },
            TextureFormat::R8 => {
                let target = self.pixels[pixel_offset];
                pixel.x = u8_to_f32(target);
                pixel.y = pixel.x;
                pixel.z = pixel.x;
            },
            _ => {
                let target = self.pixels[pixel_offset * pixel_size as usize];
                match pixel_size {
                    1 => pixel.x = u8_to_f32(target),
                    2 => {
                        pixel.x = u8_to_f32(target);
                        pixel.y = u8_to_f32(target+1);
                    },
                    3 => {
                        pixel.x = u8_to_f32(target);
                        pixel.y = u8_to_f32(target+1);
                        pixel.z = u8_to_f32(target+2);
                    },
                    4 => {
                        pixel.x = u8_to_f32(target);
                        pixel.y = u8_to_f32(target+1);
                        pixel.z = u8_to_f32(target+2);
                        pixel.w = u8_to_f32(target+3);
                    }
                    _ => panic!("Unsupported pixel type yet."),
                }
                if self.format == TextureFormat::SRGB8_A8 || self.format == TextureFormat::SRGB8 {
                    pixel.x = convert_to_linear_color(pixel.x);
                    pixel.y = convert_to_linear_color(pixel.y);
                    pixel.z = convert_to_linear_color(pixel.z);
                }
            },
        }
        pixel
    }
}
