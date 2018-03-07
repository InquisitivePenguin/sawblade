use game::msg::Msg;
use game::scene::Scene;
pub trait GameController {
    fn bind(id: u64) -> Self where Self : Sized;
    fn recv(&mut self, message: Msg) {}
    fn tick(&mut self, scene: &mut Scene) {}
}

