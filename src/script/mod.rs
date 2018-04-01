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
    fn apply_state(&mut self, state: ScriptState);
}

pub struct Script {
    lua_context: Lua
}
/// This represents the state of a Script class after running all events and systems for that tick.
pub struct ScriptState<'script> {
    state_table: Table<'script>
}


pub struct ScriptEngine {
    scriptable_entities: Vec<Box<ScriptableEntity>>,
    world: Script
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
    pub fn get_global_vars(&self) -> Table {
        self.lua_context.globals()
    }
}

impl<'script> ScriptState<'script> {
    pub fn from_script(script: &'script Script) -> ScriptState<'script> {
        ScriptState {
            state_table: script.get_global_vars()
        }
    }
    pub fn get_var<'state, T: FromLua<'state>>(&'state self, var_name: String) -> T {
        self.state_table.get::<_, T>(var_name).expect("Variable not found in ScriptState")
    }
}

impl ScriptEngine {
    pub fn new() -> ScriptEngine {
        ScriptEngine {
            scriptable_entities: vec![],
            world: Script::from_string("".to_string())
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