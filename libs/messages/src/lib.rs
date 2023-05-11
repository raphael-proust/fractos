mod point;
mod range;
mod resolution;

use std::num::NonZeroU32;

use range::Range;
use resolution::Resolution;

pub struct Task {
    pub algo: fractal::Julia,
    pub resolution: Resolution,
    pub range: Range,
    pub itermax: NonZeroU32,
}

pub struct Answer {
    pub matrix: Vec<(i32, f32)>,
}

enum Message {
    Task(Task),
    Answer(Answer),
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
