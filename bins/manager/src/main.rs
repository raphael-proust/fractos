mod libs;
use fractal::{Intensity,Julia,Fractal};
use complex::Complex;
use raylib::{color::Color, prelude::*};

fn main() {
    let (mut rl, thrd) = raylib::init().size(800, 600).title("Fractos").build();

    let mut intensities : Vec::<Intensity> = vec![];
    let j = Julia { c: Complex::new(0.3, 0.5), divergence_threshold_square: 16. };
    for x in 0..600 {
        for y in 0..800 {
            let re: f64 = (-1.0) + (2.0 * x as f64 / 600 as f64);
            let im: f64 = (-1.0) + (2.0 * y as f64 / 600 as f64);
            let c : Complex = Complex { re, im };
            let i : Intensity = j.eval(10, c);
            intensities.push(i);
      }
    };

    loop {
    let mut d = rl.begin_drawing(&thrd);
    d.clear_background(Color::WHITE);
    libs::render::render(
        800,
        600,
        &intensities,
        &libs::render::Fire,
        &mut d,
    );
    }
}
