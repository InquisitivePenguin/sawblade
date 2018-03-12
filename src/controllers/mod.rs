use std::marker::Sized;
use core::world;
pub trait Controller {
    type World : world::World;
    fn bind(id: u64) -> Self where Self : Sized;
    fn tick(&mut self, world: *mut Self::World);
}