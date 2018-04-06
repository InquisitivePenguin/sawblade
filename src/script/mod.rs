extern crate rlua;
use self::rlua::{Lua, UserData, FromLua, Table, Value, Integer, Number};
use std;
use std::fs::File;
use std::io::Read;
use core::entity::Entity;
use core::system::System;

/// This is a enum that represents a type implemented in Lua. This is primarily used in the ScriptableEntity::inject() function.
#[derive(Clone, Debug)]
pub enum LuaType {
    Nil,
    Boolean(bool),
    Integer(i64),
    Number(f64),
    String(String),
}

impl LuaType {
    fn as_value<'lua>(&self, lua_context: &'lua Lua) -> Value<'lua> {
        match self.clone() {
            LuaType::Nil => Value::Nil,
            LuaType::Boolean(val) => Value::Boolean(val),
            LuaType::Integer(int) => Value::Integer(Integer::from(int)),
            LuaType::Number(float) => Value::Number(Number::from(float)),
            LuaType::String(string) => Value::String(lua_context.create_string(string.as_str()).unwrap())
        }
    }
}
/// This is a link between an Entity and it's Lua class inside the scripting machine.
/// When the scripting machine runs events and then applies the components for the entity,
/// the values are passed back in the form of
pub trait ScriptableEntity: Entity {
    fn get_entity_script(&self) -> Script;

    fn inject(&self) -> Vec<(String, LuaType)>;

    fn apply_state(&mut self, state: ScriptState);
}

pub struct Script {
    lua_context: Lua
}
/// This represents the state of a Script class after running all events and systems for that tick.
#[derive(Clone)]
pub struct ScriptState<'script> {
    state_table: Table<'script>
}

/// This is the central manager for the World, which handles all messages, entities, and systems. It runs entity scripts and executes
/// system events.
pub struct ScriptEngine<W> {
    scriptable_entities: Vec<Box<ScriptableEntity<WorldState=W>>>,
    systems: Vec<Box<System<WorldState=W>>>,
    world_state: W
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
    pub fn set_var<'lua>(&'lua mut self, var_name: String, value: Value<'lua>) {
        self.lua_context.globals().set(var_name.as_str(), value);
    }
    pub fn ltype_to_value<'lua>(&'lua self, ltype: LuaType) -> Value<'lua> {
        ltype.as_value(&self.lua_context)
    }
}

impl<'script> ScriptState<'script> {
    pub fn from_script(script: &'script Script) -> ScriptState<'script> {
        ScriptState {
            state_table: script.get_global_vars()
        }
    }
    pub fn get<'state, T: FromLua<'state>>(&'state self, var_name: String) -> Result<T, rlua::Error> {
        self.state_table.get::<_, T>(var_name)
    }
}

impl<W> ScriptEngine<W> {
    pub fn new<WorldState>(init_state: WorldState) -> ScriptEngine<WorldState> {
        ScriptEngine {
            scriptable_entities: vec![],
            systems: vec![],
            world_state: init_state
        }
    }
    pub fn load_single<T: 'static>(&mut self, entity: T) where T: std::marker::Sized + ScriptableEntity<WorldState=W> {
        self.scriptable_entities.push(Box::new(entity));
    }
    pub fn load<T: 'static>(&mut self, entities: Vec<T>) where T : std::marker::Sized + ScriptableEntity<WorldState=W> {
        for entity in entities {
            self.load_single::<T>(entity);
        }
    }
    pub fn run(&mut self) {
        // 1. Run entity scripts
        for mut entity in &mut self.scriptable_entities {
            let mut entity_script = entity.get_entity_script();
            let injection_values = entity.inject();
            for value in injection_values {
                let var_value = {
                    entity_script.get_global_vars().0
                };
                entity_script.set_var(value.0, Value::Nil);
            }
        }
        unimplemented!()
    }
}