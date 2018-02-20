use game::gameobject::GameObject;
use game::gamedelegate::GameDelegate;
use graphics::texture::FinalTexture;
use graphics::pixel::Pixel;
pub trait Scene {
    type Input;

    fn get_final_textures(&mut self) -> Option<Vec<FinalTexture>> {None}

    fn get_raw_pixels(&mut self) -> Option<Vec<Vec<Option<Pixel>>>> {None}

    fn get_scene_entities(&mut self) -> Vec<Box<GameObject>>;

    fn get_scene_name(&self) -> &str;

    fn on_init(&mut self) {}

    fn on_tick(&mut self, input: Self::Input) -> Option<GameDelegate> {None}

    fn on_tick_internal(&mut self) {
        for mut entity in self.get_scene_entities() {
            let delegate = entity.on_tick();
        }
    }

    fn on_exit(&mut self) {}
}