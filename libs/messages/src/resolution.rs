pub struct Resolution {
    x: u32,
    y: u32,
}

impl Resolution {
    pub fn new(x: u32, y: u32) -> Option<Resolution> {
        if x == 0 || y == 0 {
            return None;
        }
        Some(Resolution { x, y })
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }
}
