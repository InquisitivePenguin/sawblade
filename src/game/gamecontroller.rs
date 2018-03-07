use game::msg::Msg;
use game::scene::Scene;
use game::gameobject::GameObject;
pub trait GameController {
    fn bind(id: u64) -> Self where Self : Sized;
    fn recv(&mut self, scene: *mut Scene, message: Msg) {}
    fn tick(&mut self, scene: *mut Scene) {}
}

