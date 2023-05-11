pub mod point;
pub mod range;
pub mod resolution;

use range::Range;
use resolution::Resolution;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU16;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub algo: fractal::Julia,
    pub resolution: Resolution,
    pub range: Range,
    pub itermax: NonZeroU16,
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
