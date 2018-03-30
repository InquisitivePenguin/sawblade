use graphics::texture::FinalTexture;
use core::entity::Entity;
pub fn render<T: Entity>(entities: &mut Vec<T>) -> Vec<FinalTexture> {
    let mut textures = vec![];
    for entity in entities {
        textures.append(&mut entity.render())
    };
    textures
}