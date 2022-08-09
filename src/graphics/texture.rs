pub enum TextureFormat {
    R8,
    RGB8,
    SRGB8,
    RGBA8,
    SRGB8_A8,
    DEPTH_FLOAT,
}

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

    pub fn set_texture_pixels(&mut self, pixels: &[u8]) -> Result<(), &str>{

        let pixel_size = match self.format {
            TextureFormat::R8 => 1,
            TextureFormat::RGB8 | TextureFormat::SRGB8 => 3,
            TextureFormat::RGBA8 | TextureFormat::SRGB8_A8 => 4,
            TextureFormat::DEPTH_FLOAT => core::mem::size_of::<f32>(),
        };

        match (self.height * self.width * pixel_size as u32) as usize >= pixels.len() {
            true => {
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

    pub fn get_texture_pixels(&self) -> &Vec<u8> {
        &self.pixels
    }

    pub fn get_shape(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    // TODO: 
    // https://github.com/zaiic/foolrenderer/blob/main/foolrenderer/graphics/texture.c
    // pub fn texture_sample() {}
}
