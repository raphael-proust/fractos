pub struct Point { }
pub struct Intensity { module: f64, divergence: u16 }

pub trait Fractal {
    fn eval(&self, _: u16, _: Point) -> Intensity {
        return Intensity {module:0., divergence:0}
    }
}
pub struct Julia { c: Point }

impl Fractal for Julia {
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
