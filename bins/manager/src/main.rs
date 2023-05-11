mod libs;
use complex::Complex;
use fractal::{Fractal, Intensity, Julia};
use raylib::{color::Color, prelude::*};

fn compute_intensities(max_iter: u16, intensities: &mut Vec<Intensity>, fractal: &impl Fractal) {
    intensities.clear();
    for x in 0..600 {
        for y in 0..800 {
            let re: f64 = (-1.0) + (2.0 * x as f64 / 600 as f64);
            let im: f64 = (-1.0) + (2.0 * y as f64 / 600 as f64);
            let c: Complex = Complex { re, im };
            let i: Intensity = fractal.eval(max_iter, c);
            intensities.push(i);
        }
    }
}

fn main() {
    let (mut rl, thrd) = raylib::init().size(800, 600).title("Fractos").build();

    let mut max_iter = 100;

    let mut intensities: Vec<Intensity> = vec![];
    let j = Julia {
        c: Complex::new(0.3, 0.5),
        divergence_threshold_square: 16.,
    };

    compute_intensities(max_iter, &mut intensities, &j);

    while !rl.window_should_close() {
        let key_opt = rl.get_key_pressed();

        match key_opt {
            None => (),
            Some(key) => match key {
                KeyboardKey::KEY_J => {
                    println!("j");
                    max_iter = (max_iter * 5) / 6;
                    compute_intensities(max_iter, &mut intensities, &j);
                }
                KeyboardKey::KEY_K => {
                    max_iter = (max_iter * 6) / 5;
                    compute_intensities(max_iter, &mut intensities, &j);
                }
                _ => (),
            },
        };

        let mut d = rl.begin_drawing(&thrd);
        d.clear_background(Color::WHITE);
        libs::render::render(800, 600, &intensities, &libs::render::Fire, &mut d);
    }
}
