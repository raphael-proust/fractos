use complex::Complex;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Intensity {
    pub module: f64,
    pub divergence: f32,
}

pub trait Fractal {
    fn eval(&self, _: u16, _: Complex) -> Intensity;
    fn into_algo(&self) -> Algo;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Julia {
    pub c: Complex,
    pub divergence_threshold_square: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Algo {
    Julia(Julia),
}

impl Julia {
    fn next(&self, x: Complex) -> Complex {
        complex::sq(x) + self.c
    }
}

impl Fractal for Julia {
    fn eval(self: &Julia, maxiter: u16, x: Complex) -> Intensity {
        let mut divergence: u16 = 0;
        let mut acc = x;
        while divergence < maxiter && complex::sqmodule(&acc) < self.divergence_threshold_square {
            divergence = divergence + 1;
            acc = self.next(acc)
        }
        let module = complex::sqmodule(&acc);
        let divergence = divergence as f32 / maxiter as f32;
        Intensity { module, divergence }
    }

    fn into_algo(&self) -> Algo {
        Algo::Julia(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn zero_fixedpoint() {
        let j = Julia {
            c: Complex::new(0., 0.),
            divergence_threshold_square: 16.,
        };
        let Intensity { module, divergence } = j.eval(10, Complex::new(0., 0.));
        assert_eq!(module, 0.);
        assert_eq!(divergence, 1.);
    }

    #[test]
    fn trivial_divergence() {
        let j = Julia {
            c: Complex::new(0., 0.),
            divergence_threshold_square: 16.,
        };
        let Intensity { module, divergence } = j.eval(10, Complex::new(2., 0.));
        assert!(module >= 16.0);
    }
}
