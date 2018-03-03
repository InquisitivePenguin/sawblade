extern crate sdl2;
use self::sdl2::render::Texture;
use self::sdl2::render::TextureCreator;
use std::cell::RefCell;
// FinalTexture is a single image that represents a raw texture to be rendered to the screen
pub struct FinalTexture {
    scene_coords: (u64,u64),
    texture: Texture
}

impl FinalTexture {
    pub fn new(tc: &TextureCreator<Texture>, coords: (u64,u64)) {

    }
}

struct GameObjectTexture {
    textures: Vec<Texture>,
    texture_files: Vec<String>
}