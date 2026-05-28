use macroquad::{
    prelude::load_texture,
    texture::{FilterMode, Texture2D},
};

pub async fn load_texture_safe(path: &str) -> Texture2D {
    match load_texture(path).await {
        Ok(loaded_texture) => loaded_texture,
        Err(e) => {
            eprintln!("texture {} failed to load: {} Continue with error texture.", path, e);
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
