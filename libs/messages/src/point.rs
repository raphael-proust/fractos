pub struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Option<Point> {
        if x.is_nan() || y.is_nan() {
            return None;
        }
        Some(Point { x, y })
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }
}
