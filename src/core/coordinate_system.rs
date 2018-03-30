pub struct CoordinateSystem {
    max_x: u64,
    max_y: u64,
    scale: u32
}

impl CoordinateSystem {
    pub fn new(max_x: u64, max_y: u64, scale: u32) -> CoordinateSystem {
        CoordinateSystem {
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