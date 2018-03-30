use core::event::Event;
pub struct MessageHandler;

pub enum Message {
    Input(Event),
    Custom(String)
}

impl From<Event> for Message {
    fn from(event: Event) -> Message {
        Message::Input(event)
    }
}