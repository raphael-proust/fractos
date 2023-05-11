use crate::point::Point;

pub struct Range {
    min: Point,
    max: Point,
}

impl Range {
    pub fn new(min: Point, max: Point) -> Option<Range> {
        if min.x() >= max.x() || min.y() >= max.y() {
            None
        } else {
            Some(Range { min, max })
        }
    }

    pub fn min(r: Range) -> Point {
        r.min
    }

    pub fn max(r: Range) -> Point {
        r.max
    }
}
