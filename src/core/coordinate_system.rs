/// This is a helper class for coordinate systems. It acts as a container for coordinate system data and provides
/// useful functions.
pub struct CoordinateSystemData {
    max_x: u64,
    max_y: u64,
    scale: u32
}

impl CoordinateSystemData {
    pub fn new(max_x: u64, max_y: u64, scale: u32) -> CoordinateSystemData {
        CoordinateSystemData {
            max_x,
            max_y,
            scale
        }
    }
    pub fn move_to(current: (u32, u32), x_movement: i32, y_movement: i32) -> (u32,u32) {
        let mut x = 0;
        if current.0 as i32 + x_movement > 0 {
            x = current.0 as i32 + x_movement;
        }
        let mut y = 0;
        if current.1 as i32 + y_movement > 0 {
            y = current.1 as i32 + y_movement;
        }
        (x as u32,y as u32)
    }
}

/// This is a System that can easily be added to a ScriptEngine for handling of movement. It receives messages that move entities around
/// safely.
struct CoordinateSystem {
  data: CoordinateSystemData
}

