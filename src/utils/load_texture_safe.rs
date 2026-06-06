use macroquad::prelude::Image;
use macroquad::texture::{FilterMode, Texture2D};

pub fn load_texture_safe(bytes: &[u8]) -> Texture2D {
    match Image::from_file_with_format(bytes, None) {
        Ok(loaded_image) => Texture2D::from_image(&loaded_image),
        Err(e) => {
            eprintln!("texture failed to load: {} Continue with error texture.", e);
            let magenta = [255, 0, 255, 255];
            let black = [0, 0, 0, 255];
            let mut pixels = Vec::new();
            pixels.extend_from_slice(&magenta);
            pixels.extend_from_slice(&black);
            pixels.extend_from_slice(&black);
            pixels.extend_from_slice(&magenta);
            let texture = Texture2D::from_rgba8(2, 2, &pixels);
            texture.set_filter(FilterMode::Nearest);
            texture
        }
    }
}
