use std::ops::*;
/// The Vector object is useful for representing 2D number pairs, such as coordinates, velocities, and other pairs.
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
    pub fn from_generic(generic_vec: (impl Into<u64>, impl Into<u64>)) -> Vector {
        Vector {
            x : generic_vec.0.into(),
            y: generic_vec.1.into()
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

    pub fn dot(&self, other: &Vector) -> u64 {
        self.x * other.x + self.y * other.y
    }
}

impl From<(u32,u32)> for Vector {
    fn from(tuple: (u32,u32)) -> Vector {
        Vector {
            x: tuple.0.into(),
            y: tuple.1.into()
        }
    }
}

impl Add for Vector {
    type Output = Vector;
    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Sub for Vector {
    type Output = Vector;
    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl Mul for Vector {
    type Output = Vector;
    fn mul(self, other: Vector) -> Vector {
        Vector {
            x: self.x * other.x,
            y: self.y * other.y
        }
    }
}

impl Div for Vector {
    type Output = Vector;
    fn div(self, other: Vector) -> Vector {
        Vector {
            x: self.x / other.x,
            y: self.y / other.y
        }
    }
}