mod libs;
use complex::Complex;
use fractal::{Fractal, Intensity, Julia};
use raylib::{color::Color, prelude::*};

const XRES: u32 = 800;
const YRES: u32 = 600;

fn compute_intensities(max_iter: u16, intensities: &mut Vec<Intensity>, fractal: &impl Fractal) {
    intensities.clear();
    for y in 0..YRES {
        for x in 0..XRES {
            let c: Complex = complex_of_window_position(x, y, XRES, YRES);
            let i: Intensity = fractal.eval(max_iter, c);
            intensities.push(i);
        }
    }
}

fn complex_of_window_position(xpos: u32, ypos: u32, xres: u32, yres: u32) -> Complex {
    let re: f64 = (-1.0) + (2.0 * xpos as f64 / xres as f64);
    let im: f64 = (-1.0) + (2.0 * ypos as f64 / yres as f64);
    let c: Complex = Complex { re, im };
    c
}

fn main() {
    let (mut rl, thrd) = raylib::init().size(800, 600).title("Fractos").build();

    let mut max_iter = 100;

    let mut intensities: Vec<Intensity> = vec![];

    let fractal_params = Complex::new(0.3, 0.5);

    let mut fractal = Julia {
        c: fractal_params,
        divergence_threshold_square: 16.,
    };

    compute_intensities(max_iter, &mut intensities, &fractal);

    let mut dirty = false;

    while !rl.window_should_close() {
        let key_opt = rl.get_key_pressed();

        dirty = false;

        match key_opt {
            None => (),
            Some(key) => match key {
                KeyboardKey::KEY_J => {
                    println!("j");
                    max_iter = (max_iter * 5) / 6;
                    dirty = true;
                }
                KeyboardKey::KEY_K => {
                    max_iter = (max_iter * 6) / 5;
                    dirty = true;
                }
                _ => (),
            },
        };

        let is_mouse_down = rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON);

        if is_mouse_down {
            dirty = true;
            let mouse_pos = rl.get_mouse_position();
            let xpos = mouse_pos.x as u32;
            let ypos = mouse_pos.y as u32;
            fractal.c = complex_of_window_position(xpos, ypos, XRES, YRES);
        }

        if dirty {
            compute_intensities(max_iter, &mut intensities, &fractal);
        }

        let mut d = rl.begin_drawing(&thrd);
        d.clear_background(Color::WHITE);
        libs::render::render(XRES, YRES, &intensities, &libs::render::Fire, &mut d);
    }
}
