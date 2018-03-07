use graphics::texture::FinalTexture;
use std::cell::Ref;

pub trait GameObject {
    /*
    spawn should be called when creating a new GameObject-based entity. Don't spawn if function returns false
    */
    fn spawn(coordinates: (u32,u32), id: u64) -> Self where Self : Sized;
    fn get_id(&self) -> u64;
    fn recv(&mut self, trigger: String) {}
    fn render(&mut self) -> Option<FinalTexture> {None}
}

pub trait GameEntity : GameObject {
    fn get_coordinates(&self) -> (u32,u32);
    fn get_bounding_box(&self) -> (u32,u32);
}