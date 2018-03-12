use std::collections::HashMap;
use core::event::Event;
use graphics::texture::FinalTexture;

pub trait World {
    fn init(&mut self) {}
    fn event_loop(&mut self, _events: Vec<Event>) -> Vec<FinalTexture>;
}