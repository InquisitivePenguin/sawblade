pub trait Scene {
    type ObjectContainer;
    type FinalTextureContainer;
    type PixelContainer;
    type GameDelegate;
    type Input;

    fn run_tick(&mut self, input: Self::Input) -> Self::GameDelegate;

    fn f_texture_list(&mut self) -> Self::FinalTextureContainer;

    fn raw_pixel_list(&mut self) -> Self::PixelContainer;
}