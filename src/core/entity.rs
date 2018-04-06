use graphics::texture::FinalTexture;
use core::world;
use core::event::Event;
use script::ScriptableEntity;
use core::utils::Message;

pub trait Entity {
    type WorldState;
    fn get_id(&self) -> u64;
    fn render(&mut self) -> Vec<FinalTexture> { vec![] }
    fn handle_msg(&mut self, _msg: Message) {}
}