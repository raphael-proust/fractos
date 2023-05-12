mod libs;
use clap::Parser;
use complex::Complex;
use fractal::{Algo, Fractal, Intensity, Julia, Mandelbrot};
use libs::tasks_manager;
use libs::tasksplitter;
use messages::{point, range, resolution::Resolution, Answer, Task};
use raylib::{color::Color, prelude::*};
use std::io::Read;
use std::io::Write;
use std::num::NonZeroU16;
use std::num::NonZeroU32;
use tokio::net::{TcpListener, TcpStream};
use worker;

fn render(
    xres: u32,
    yres: u32,
    iter_max: u16,
    fractal: &impl Fractal,
    xblocks: u32,
    yblocks: u32,
    block_size: u32,
    quad_exp: u8,
    d: &mut RaylibDrawHandle,
) {
    let tasks = tasksplitter::split_quad(iter_max, fractal, xblocks, yblocks, block_size, quad_exp);
    let quad_fact = 1 << quad_exp;
    for (task, xindex, yindex) in tasks {
        let Answer { matrix } = worker::handle_task(&task);
        let xres = xres / quad_fact;
        let yres = yres / quad_fact;
        let xoffs = 0 + xindex * xres;
        let yoffs = 0 + yindex * yres;
        libs::render::render_averaged_chunk(
            xoffs,
            yoffs,
            xres,
            yres,
            block_size,
            &matrix,
            &libs::render::Wow,
            d,
        );
    }
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
        _ => panic!(),
    }
}

async fn create_and_send_dummy_julia_task(stream: &mut TcpStream) {
    println!("Create dummy task");
    let fractal_params = Complex::new(0.3, 0.5);
    let x = NonZeroU32::new(2).unwrap();
    let y = NonZeroU32::new(2).unwrap();
    let resolution: Resolution = Resolution { x, y };
    let min = point::Point::new(1_f64, 2_f64).unwrap();
    let max = point::Point::new(3_f64, 4_f64).unwrap();
    let range = range::Range::new(min, max).unwrap();
    let algo = fractal::Algo::Julia(Julia {
        c: fractal_params,
        divergence_threshold_square: 16.,
    });
    let itermax = NonZeroU16::new(2).unwrap();
    let task = Task {
        algo,
        resolution,
        range,
        itermax,
    };
    let mut stream = stream;
    println!("Will call send task");
    tasks_manager::send_task(&mut stream, task).await;
    println!("Task sended, awaiting answer");
    tasks_manager::read_answer(&mut stream).await;
    println!("Answer received");
}

async fn create_and_send_mandelbrot_task(stream: &mut TcpStream) {
    println!("Create dummy task");
    let fractal_params = Complex::new(0.3, 0.5);
    let x = NonZeroU32::new(2).unwrap();
    let y = NonZeroU32::new(2).unwrap();
    let resolution: Resolution = Resolution { x, y };
    let min = point::Point::new(1_f64, 2_f64).unwrap();
    let max = point::Point::new(3_f64, 4_f64).unwrap();
    let range = range::Range::new(min, max).unwrap();
    let algo = fractal::Algo::Mandelbrot(Mandelbrot { c: fractal_params });
    let itermax = NonZeroU16::new(2).unwrap();
    let task = Task {
        algo,
        resolution,
        range,
        itermax,
    };
    let mut stream = stream;
    println!("Will call send task");
    tasks_manager::send_task(&mut stream, task).await;
    println!("Task sended, awaiting answer");
    tasks_manager::read_answer(&mut stream).await;
    println!("Answer received");
}

#[tokio::main]
async fn main() {
    let args = libs::args::Args::parse();

    let mut max_iter = args.max_iter;
    let starting_block_size = 10;
    let mut block_size = starting_block_size;
    let xres = args.resolution.xres;
    let yres = args.resolution.yres;
    let quad_exp = 1;

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

    let listener = TcpListener::bind("localhost:4242").await.unwrap();

    let mut xblocks = xres / block_size;
    let mut yblocks = yres / block_size;
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let mut stream = stream;
        create_and_send_dummy_julia_task(&mut stream).await;
        //create_and_send_mandelbrot_task(&mut stream).await;
        let mut dirty = true;

        while !rl.window_should_close() {
            let key_opt = rl.get_key_pressed();
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
                    KeyboardKey::KEY_S => {
                        println!("Saving to \"out.png\"");
                        rl.take_screenshot(&thrd, "out.png");
                    }
                    _ => (),
                },
            };

            let new_mouse_pos = rl.get_mouse_position();
            if mouse_pos != new_mouse_pos {
                dirty = true;
                block_size = starting_block_size;
                xblocks = xres / block_size;
                yblocks = yres / block_size;
                mouse_pos = new_mouse_pos;
                fractal.c =
                    complex_of_window_position(mouse_pos.x as u32, mouse_pos.y as u32, xres, yres);
            }

            if !dirty && block_size > 1 {
                dirty = true;
                block_size = next_block_size_down(block_size);
                xblocks = xres / block_size;
                yblocks = yres / block_size;
            }

            let mut d = rl.begin_drawing(&thrd);
            if dirty {
                d.clear_background(Color::WHITE);
                render(
                    xres, yres, max_iter, &fractal, xblocks, yblocks, block_size, quad_exp, &mut d,
                );
                dirty = false;
            }
        }
    }
}
