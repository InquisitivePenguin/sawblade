use graphics::graphicalcontext::GraphicalContext;
/// Renderer is a useful trait for modulated rendering onto the screen. One advantage of Renderers
/// is that Sawblade can handle UI elements defined by a UIState using an internal UIRenderer, reducing boilerplate. Other uses,
/// besides UI, are good for handling shaders, menu bars, and graphical effects.
pub trait Renderer {
    type GameState;
    fn render(&mut self, state: &Self::GameState, context: &mut GraphicalContext);
}