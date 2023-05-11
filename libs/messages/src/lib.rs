mod point;

pub struct Resolution {
    x: u32,
    y: u32,
}

pub struct Task {
    algo: String,
    resolution: Resolution,
    range: Range,
    itermax: u32,
}

pub struct Answer {
    matrix: Vec<(i32, f32)>,
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
