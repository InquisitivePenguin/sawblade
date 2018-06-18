/// The Entity object is part 1 of the ECS (Entity, Component, System) model
#[derive(Clone, Debug)]
pub struct Entity {
    id: u64,
    /// The first part of the tuple is the Component name, the later part is the serialized component data
    components: Vec<(String, String)>
}

impl Entity {
    pub fn register_blank(id: u64) -> Entity {
        Entity {
            id,
            components : vec!()
        }
    }
    pub fn register(id: u64, components: Vec<(String, String)>) -> Entity {
        Entity {
            id,
            components
        }
    }
}