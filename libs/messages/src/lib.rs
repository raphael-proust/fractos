pub mod point;
pub mod range;
pub mod resolution;

use fractal;
use range::Range;
use resolution::Resolution;
use serde::{Deserialize, Serialize};
use std::num::{NonZeroU16, NonZeroU32};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub algo: fractal::Algo,
    pub resolution: Resolution,
    pub range: Range,
    pub itermax: NonZeroU16,
}

impl Task {
    pub fn new(
        algo: fractal::Algo,
        resx: u32,
        resy: u32,
        rp0x: f64,
        rp0y: f64,
        rp1x: f64,
        rp1y: f64,
        itermax: u16,
    ) -> Task {
        Task {
            algo,

            resolution: Resolution {
                x: NonZeroU32::new(resx).unwrap(),
                y: NonZeroU32::new(resy).unwrap(),
            },
            range: Range::new(
                point::Point::new(rp0x, rp0y).unwrap(),
                point::Point::new(rp1x, rp1y).unwrap(),
            )
            .unwrap(),
            itermax: NonZeroU16::new(itermax).unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Answer {
    pub matrix: Vec<fractal::Intensity>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
