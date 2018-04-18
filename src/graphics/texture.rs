use core::math::Vector;
/// This structure is not yet implemented
pub struct SpriteMap;
/// A container class for TextureComponents that provide helpful utilities for rendering
pub struct Texture {
    pub components: Vec<TextureComponent>,
    pub relative_origin: Vector
}
/// The TextureComponent represents a piece of a texture that is rendered to the screen
pub enum TextureComponent {
    BasicShape(Shape, Color),
    Image(String)
}
pub enum Shape {
    Rectangle(Vector),
    Circle(u64),
    Triangle(Vector, Vector, Vector)
}
// Represents the color of a shape
type Color = (u8,u8,u8);