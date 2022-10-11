use super::{texture::{Texture, TextureFormat}, color::f32_to_u8};
use crate::math::{utility::f32_clamp01, preclude::u32_min};

static mut CLEAR_COLOR: [u8; 4] = [0; 4];

#[derive(PartialEq, Eq, Debug)]
pub enum AttachmentType {
    Color,
    Depth,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct FrameBuffer {
    width: u32,
    height: u32,
    color_buffer: Option<Box<Texture>>,
    depth_buffer: Option<Box<Texture>>,
}

impl FrameBuffer {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            color_buffer: None,
            depth_buffer: None,
        }
    }

    pub fn attach_texture(&mut self, attachment: AttachmentType, texture: Option<Box<Texture>>) {
        let format = texture.as_ref().unwrap().get_texture_format();
        let mut result = false;
        if let Some(texture) = texture {
            match attachment {
                AttachmentType::Color => {
                    if format == TextureFormat::SRGB8_A8 || format == TextureFormat::RGBA8 {
                        self.color_buffer = Some(texture);
                        result = true;
                    }
                },
                AttachmentType::Depth => {
                    if format == TextureFormat::DEPTH_FLOAT {
                        self.depth_buffer = Some(texture);
                        result = true;
                    }
                },
            }
        } else {
            match attachment {
                AttachmentType::Color => {
                    if format == TextureFormat::SRGB8_A8 || format == TextureFormat::RGBA8 {
                        self.color_buffer = None;
                        result = true;
                    }
                },
                AttachmentType::Depth => {
                    if format == TextureFormat::DEPTH_FLOAT {
                        self.depth_buffer = None;
                        result = true;
                    }
                },
            }
        }
        if result {
            if self.color_buffer.is_none() && self.depth_buffer.is_none() {
                self.width = 0;
                self.height = 0;
            } else {
                self.width = std::u32::MAX;
                self.height = std::u32::MIN;

                // FIXME:
                // let color_buf = self.color_buffer;
                // let depth_buf = self.depth_buffer;
                // self.set_min_size(color_buf);
                // self.set_min_size(depth_buf);
            }
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_hegiht(&self) -> u32 {
        self.height
    }

    pub fn get_attachment(&self, attachment: AttachmentType) -> &Option<Box<Texture>> {
        let result = match attachment {
            AttachmentType::Color => {
                &self.color_buffer
            },
            AttachmentType::Depth => {
                &self.depth_buffer
            },
        };

        result 
    }

    pub fn clear(&mut self) {
        let pixel_count: usize = (self.width * self.height).try_into().unwrap();
        if let Some(ref mut buffer) = self.color_buffer {
            let pixels = buffer.get_texture_pixels();
            let mut i = 0;
            while i < pixel_count {
                let ii = i * 4;
                unsafe {
                    pixels[ii] = CLEAR_COLOR[0];
                    pixels[ii + 1] = CLEAR_COLOR[1];
                    pixels[ii + 2] = CLEAR_COLOR[2];
                    pixels[ii + 3] = CLEAR_COLOR[3];
                }
                i += 1;
            }
        }
        if let Some(ref mut buffer) = self.depth_buffer {
            let pixels = buffer.get_texture_pixels();
            let mut i = 0;
            while i < pixel_count {
                pixels[i] = 1;
                i += 1;
            }
        }
    }

    #[inline]
    fn set_min_size(&mut self, buffer: &Option<Box<Texture>>) {
        let (buf_width, buf_height) = buffer.as_ref().unwrap().get_shape();
        self.width = u32_min(self.width, buf_width);
        self.height = u32_min(self.height, buf_height);
    }
}

pub fn set_clean_color(red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe {
        CLEAR_COLOR[0] = f32_to_u8(f32_clamp01(red));
        CLEAR_COLOR[1] = f32_to_u8(f32_clamp01(green));
        CLEAR_COLOR[2] = f32_to_u8(f32_clamp01(blue));
        CLEAR_COLOR[3] = f32_to_u8(f32_clamp01(alpha));
    }
}










