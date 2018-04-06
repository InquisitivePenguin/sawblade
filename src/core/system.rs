use core::entity::Entity;
use core::utils::Message;
pub trait System {
    type WorldState;
    fn tick(&mut self, messages: Vec<Message>, entities: Vec<Box<Entity<WorldState=Self::WorldState>>>, state: Self::WorldState);
}
