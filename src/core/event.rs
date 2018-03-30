use core::input::KeyboardKey;
#[derive(PartialEq, Clone, Debug)]
pub enum Event {
    Tick,
    Unrecognized,
    Key(KeyboardKey),
    MouseMovement((i32,i32)),
    Close
}