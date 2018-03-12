use graphics::texture::FinalTexture;
use core::world;

pub trait Entity {
    type World: world::World;
    fn spawn(coordinates: (u32,u32), id: u64) -> Self where Self : Sized;
    fn get_id(&self) -> u64;
    fn recv(&mut self, _scene: *mut Self::World, _trigger: String) {}
    fn render(&mut self) -> Option<FinalTexture> {None}
    fn tick(&mut self, _scene: *mut Self::World) {}
}