extern crate rlua;
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

pub struct ScriptEngine {
    scriptable_entities: Vec<Box<ScriptableEntity>>,
    world_state: WorldState
}

impl ScriptEngine {
    pub fn new() -> ScriptEngine {
        ScriptEngine {
            scriptable_entities: vec![],
            world_state: WorldState {}
        }
    }
    pub fn load_single<T: 'static>(&mut self, entity: T) where T: std::marker::Sized + ScriptableEntity {
        self.scriptable_entities.push(Box::new(entity));
    }
    pub fn load<T: 'static>(&mut self, entities: Vec<T>) where T : std::marker::Sized + ScriptableEntity {
        for entity in entities {
            self.load_single::<T>(entity);
        }
    }
    pub fn run(&mut self) {
        unimplemented!()
    }
}