use super::*;

#[test]
fn texture_impl_normal() {
    let format = texture::TextureFormat::R8;
    let mut texture = texture::Texture::new(format, 100, 100).unwrap();
    assert_eq!(texture.pixels.len(), 100 * 100);
    assert_eq!(texture.get_shape(), (100, 100));
    assert_eq!(texture.get_pixel_size(), 1);
    let pixels = vec![255; (100 * 100 * 1) as usize];
    assert_eq!(texture.get_texture_pixels(), &pixels);

    let pixels = vec![0; (100 * 100 * 1) as usize];
    texture.set_texture_pixels(&pixels).unwrap();
    assert_eq!(texture.get_texture_pixels(), &pixels);
}

