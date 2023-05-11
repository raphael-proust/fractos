#[derive(Clone)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

pub fn sqmodule(x: &Complex) -> f64 {
    (x.re * x.re) + (x.im * x.im)
}

pub fn sq(x: Complex) -> Complex {
    Complex {
        re: (x.re * x.re) - (x.im * x.im),
        im: 2. * (x.re * x.im),
    }
}
pub fn add(x: Complex, c: &Complex) -> Complex {
    Complex {
        re: x.re + c.re,
        im: x.im + x.im,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
