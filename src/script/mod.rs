use std;
use core::entity::Entity;
pub trait ScriptableEntity: Entity {
    fn get_identifier(&self) -> String;
    fn get_entity_script(&self) -> Script;
    fn apply_state(&mut self, state: EntityState);
}

pub struct Script;

pub struct EntityState;

pub struct ComponentState;

pub struct WorldState;

pub struct ScriptEngine;

impl ScriptEngine {
    pub fn new() {
        unimplemented!()
    }
    pub fn load_single<T>(&mut self, entity: T) where T: std::marker::Sized + ScriptableEntity {
        unimplemented!()
    }
    pub fn load<T>(&mut self, entities: Vec<T>) where T : std::marker::Sized + ScriptableEntity {
        unimplemented!()
    }
    pub fn apply(&mut self, world_state: WorldState) {
        unimplemented!()
    }
}