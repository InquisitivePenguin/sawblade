extern crate sawblade;
use sawblade::script::*;

fn main() {
    let some_script = Script::from_file("examples/test_entity.lua".to_string());
    let some_state = ScriptState::from_script(&some_script);
    println!("{}", some_state.get::<String>("something".to_string()).unwrap())
}