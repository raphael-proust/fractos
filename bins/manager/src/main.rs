mod libs;
use clap::Parser;
use complex::Complex;
use fractal::{Fractal, Intensity, Julia};
use messages::{Answer, Task};
use raylib::{color::Color, prelude::*};
use worker;

const XRES: u32 = 800;
const YRES: u32 = 600;

fn compute_intensities(iter_max: u16, intensities: &mut Vec<Intensity>, fractal: &impl Fractal) {
    intensities.clear();
    let algo = fractal.clone().into_algo();
    let task = Task::new(algo, XRES, YRES, -1.0, -1.0, 1.0, 1.0, iter_max);
    let Answer {
        matrix: mut par_result,
    } = worker::handle_task(&task);
    intensities.append(&mut par_result);
}

fn complex_of_window_position(xpos: u32, ypos: u32, xres: u32, yres: u32) -> Complex {
    let re: f64 = (-1.0) + (2.0 * xpos as f64 / xres as f64);
    let im: f64 = (-1.0) + (2.0 * ypos as f64 / yres as f64);
    let c: Complex = Complex { re, im };
    c
}

fn main() {
    let args = libs::args::Args::parse();

    let (mut rl, thrd) = raylib::init().size(800, 600).title("Fractos").build();

    let mut max_iter = args.max_iter;

    let mut intensities: Vec<Intensity> = vec![];

    let mut fractal = Julia {
        c: args.julia_constant,
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
