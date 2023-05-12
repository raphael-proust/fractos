mod libs;
use clap::Parser;
use complex::Complex;
use fractal::{Fractal, Intensity, Julia};
use messages::{Answer, Task};
use raylib::{color::Color, prelude::*};
use worker;

fn compute_intensities(xres: u32, yres: u32, iter_max: u16, intensities: &mut Vec<Intensity>, fractal: &impl Fractal, xblocks: u32, yblocks: u32, block_size: u32,) {
    intensities.clear();
    let algo = fractal.clone().into_algo();
    let taskxres = xres / block_size;
    let taskyres = yres / block_size;
    let re_start = -1.0 + (2. / taskxres as f64);
    let im_start = -1.0 + (2. / taskyres as f64);
    let re_end = 1.0 + (2. / taskxres as f64);
    let im_end = 1.0 + (2. / taskyres as f64);
    let task = Task::new(algo, taskxres, taskyres, re_start, im_start, re_end, im_end, iter_max);
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

fn next_block_size_down(b: u32) -> u32 {
    match b {
        20 => 10,
        10 => 5,
        5 => 2,
        2 => 1,
        1 => 1,
        _ => panic!()
    }
}

fn main() {
    let args = libs::args::Args::parse();

    let mut max_iter = args.max_iter;
    let starting_block_size = 10;
    let mut block_size = starting_block_size;
    let xres = args.resolution.xres;
    let yres = args.resolution.yres;

    let (mut rl, thrd) = raylib::init()
        .size(xres as i32, yres as i32)
        .title("Fractos")
        .build();
    let mut mouse_pos = rl.get_mouse_position();

    let mut intensities: Vec<Intensity> = vec![];

    let mut fractal = Julia {
        c: args.julia_constant,
        divergence_threshold_square: 16.,
    };


    let mut xblocks = xres / block_size;
    let mut yblocks = yres / block_size;

    compute_intensities(
        xres,
        yres,
        max_iter,
        &mut intensities,
        &fractal,
        xblocks,
        yblocks,
        block_size,
    );

    let mut dirty;

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

        let new_mouse_pos =  rl.get_mouse_position();
        if mouse_pos != new_mouse_pos {
            dirty = true;
            block_size = starting_block_size;
            mouse_pos = new_mouse_pos;
            fractal.c = complex_of_window_position(mouse_pos.x as u32, mouse_pos.y as u32, xres, yres);
        }

        if dirty {
            compute_intensities(
                xres,
                yres,
                max_iter,
                &mut intensities,
                &fractal,
                xblocks,
                yblocks,
                block_size,
            );
        } else if block_size > 1 {
            block_size = next_block_size_down(block_size);
            compute_intensities(
                xres,
                yres,
                max_iter,
                &mut intensities,
                &fractal,
                xblocks,
                yblocks,
                block_size,
            );
        }

        let mut d = rl.begin_drawing(&thrd);
        d.clear_background(Color::WHITE);
        libs::render::render_averaged_chunk(
            0,
            0,
            xres,
            yres,
            block_size,
            &intensities,
            &libs::render::Grayscale,
            &mut d,
        );
    }
}
