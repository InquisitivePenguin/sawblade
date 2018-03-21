use graphics::texture::FinalTexture;
use core::world;
use core::event::Event;

pub trait Entity {
    type World: world::World;
    fn spawn(coordinates: (u32,u32), id: u64) -> Self where Self : Sized;
    fn get_id(&self) -> u64;
    fn event(&mut self, _world: &mut Self::World, _event: Event) {}
}