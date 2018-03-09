use graphics::texture::FinalTexture;
use game::world::World;

pub trait Entity {
    /*
    spawn should be called when creating a new GameObject-based entity. Don't spawn if function returns false
    */
    fn spawn(coordinates: (u32,u32), id: u64) -> Self where Self : Sized;
    fn get_id(&self) -> u64;
    fn recv(&mut self, _scene: *mut World, _trigger: String) {}
    fn render(&mut self) -> Option<FinalTexture> {None}
    fn tick(&mut self, _scene: *mut World) {}
}