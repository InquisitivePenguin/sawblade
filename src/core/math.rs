#[derive(Clone, Copy, Debug)]
pub struct Vector {
    pub x: u64,
    pub y: u64
}

impl Vector {
    pub fn new(x: u64, y: u64) -> Vector {
        Vector {
            x,y
        }
    }
    pub fn x_u32(&self) -> u32 {
        self.x as u32
    }
    pub fn y_u32(&self) -> u32 {
        self.y as u32
    }
    pub fn x_i64(&self) -> i64 {
        self.x as i64
    }
    pub fn y_i64(&self) -> i64 {
        self.y as i64
    }
    pub fn x_i32(&self) -> i32 {
        self.x as i32
    }
    pub fn y_i32(&self) -> i32 {
        self.y as i32
    }
}
