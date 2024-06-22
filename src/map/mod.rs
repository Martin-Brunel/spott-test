
#[derive(Debug, Clone)]
pub struct Map {
    /// map width
    n: i32,
    /// map height
    m: i32
}

impl Map {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            n: width,
            m: height
        }
    }

    pub fn not_lost(&self, x: i32, y: i32) -> bool {
        return (x < 0 || x > self.n) || (y < 0 || y > self.m)
    }
}