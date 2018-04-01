extern crate sawblade;
use sawblade::script::*;

fn main() {
    let some_script = Script::from_file("examples/test_entity.lua".to_string());
    println!("{}", some_script.get_var::<String>("something".to_string()));
}