use core::input::KeyboardKey;
#[derive(PartialEq, Clone)]
pub enum Event {
    Tick,
    Key(KeyboardKey),
    Custom(String),
    MouseMovement((i32,i32)),
    Close
}