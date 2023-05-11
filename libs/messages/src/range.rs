use crate::point::Point;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
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

    pub fn min(&self) -> Point {
        self.min
    }

    pub fn max(&self) -> Point {
        self.max
    }
}
