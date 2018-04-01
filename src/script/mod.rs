extern crate rlua;
use self::rlua::{Lua, UserData, FromLua, Table};
use std;
use std::fs::File;
use std::io::Read;
use core::entity::Entity;
/// This is a link between an Entity and it's Lua class inside the scripting machine.
/// When the scripting machine runs events and then applies the components for the entity,
/// the values are passed back in the form of
pub trait ScriptableEntity: Entity {
    fn get_identifier(&self) -> String;
    fn get_entity_script(&self) -> Script;
    fn apply_state(&mut self, state: EntityState);
}

pub struct Script {
    lua_context: Lua
}
/// This represents the state of a Entity class after running all events and systems for that tick.
pub struct EntityState;

/// This represents the state of a Component class after running all events
pub struct ComponentState;

/// This represents the state of the World, which is passed around inside Lua
pub struct WorldState;


pub struct ScriptEngine {
    scriptable_entities: Vec<Box<ScriptableEntity>>,
    world_state: WorldState
}

impl Script {
    pub fn from_string(string: String) -> Script {
        let lua_context = Lua::new();
        lua_context.exec::<()>(string.as_str(), None);
        Script {
            lua_context
        }
    }
    pub fn from_file(filename: String) -> Script {
        let mut file = File::open(filename.clone()).expect(("Could not open script file: ".to_string() + filename.as_str()).as_str());
        let mut f_contents = String::new();
        file.read_to_string(&mut f_contents).expect(("Could not read from script file: ".to_string() + filename.as_str()).as_str());
        Script::from_string(f_contents)
    }
    pub fn get_var<'lua, T: FromLua<'lua>>(&'lua self, var_name: String) -> T {
        let globals = self.lua_context.globals();
        globals.get::<_, T>(var_name).unwrap()
    }
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