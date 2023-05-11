use complex::Complex;

pub struct Intensity { module: f64, divergence: u16 }

pub trait Fractal {
    fn eval(&self, _: u16, _: Complex) -> Intensity {
        return Intensity {module:0., divergence:0}
    }
}
pub struct Julia { c: Complex, divergence_threshold_square: f64 }

impl Julia {
    fn next(&self, x:Complex) -> Complex {
        complex::add(complex::sq(x), &self.c)
    }
}

impl Fractal for Julia {
    fn eval(self:&Julia, maxiter:u16, x:Complex) -> Intensity {
        let mut divergence : u16 = 0;
        let mut acc = x.clone();
        while divergence < maxiter && complex::sqmodule(&acc) < self.divergence_threshold_square {
            divergence = divergence + 1;
            acc = self.next(acc)
        };
        let module = complex::sqmodule(&acc);
        Intensity { module, divergence }
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
