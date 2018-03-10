// FinalTexture is a single image that represents a raw texture to be rendered to the screen
#[derive(Debug)]
pub struct FinalTexture {
    scene_coords: (u32,u32),
    texture: SawbladeTexture
}

impl FinalTexture {
    pub fn make_rect(size: (u32,u32), coordinates: (u32,u32)) -> FinalTexture {
        FinalTexture {
            scene_coords: coordinates,
            texture: SawbladeTexture::Rect(size.0,size.1)
        }
    }
    pub fn make_circle(radius: u32, center_coordinates: (u32,u32)) -> FinalTexture {
        FinalTexture {
            scene_coords: center_coordinates,
            texture: SawbladeTexture::Circle(radius)
        }
    }
    pub fn get_coordinates(&self) -> (u32,u32) {
        self.scene_coords
    }
    pub fn get_texture(&self) -> SawbladeTexture {
        self.texture
    }
}

#[derive(Copy, Clone, Debug)]
pub enum SawbladeTexture {
    Rect(u32,u32),
    Circle(u32),
    FromFile(&'static str)
}