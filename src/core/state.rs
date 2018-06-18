use core::entity::Entity;
/// Basically required if you are using an ECS model in your game. This will make your life much easier, as it allows
/// for Sawblade to make your life easier with it's built-in ECS tools.
pub trait ECSState {
    type Components;
    /// A full list of all entities in the game
    fn entities(&self) -> Vec<&Entity>;
    /// A list of all entities in the game that have a specific component
    fn valid_entities(&self, component_name: &str) -> Vec<&Entity>;
    /// A mutable reference to an entity
    fn mut_entity_ref(&mut self, id: u64) -> Option<&mut Entity>;
}

pub trait UIState {
    /// Not yet defined; should return references to all valid UI components and their layers (for drawing order)
    fn get_ui_components(&self) -> Vec<()>;
}