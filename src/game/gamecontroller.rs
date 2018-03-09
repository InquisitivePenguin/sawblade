use game::msg::Msg;
use game::world::World;
pub trait Controller {
    fn bind(id: u64) -> Self where Self : Sized;
    fn recv(&mut self, _scene: *mut World, _message: Msg) {}
    fn tick(&mut self, _scene: *mut World) {}
}

