use fractal::Intensity;

use raylib::{color::Color, prelude::*};

pub trait ColorMap {
    fn of_intensity(&self, intensity: &Intensity) -> Color;
}

pub struct Grayscale;

impl ColorMap for Grayscale {
    fn of_intensity(&self, intensity: &Intensity) -> Color {
        let Intensity { .. } = intensity;
        todo!();
    }
}

// pub fn init_display(x: u32, y: u32) -> RaylibDrawHandle {
//     let (mut rl, thrd) = raylib::init().size(x, y).title("BIH").build();

//     let mut d = rl.begin_drawing(&thrd);
//     d.clear_background(Color::WHITE);
//     d
// }

pub fn render(
    xres: u32,
    yres: u32,
    intensities: &Vec<Intensity>,
    color_map: &impl ColorMap,
    d: &mut RaylibDrawHandle,
) -> () {
    for x in 0..xres {
        for y in 0..yres {
            let index: usize = (x + y * xres) as usize;
            let c = color_map.of_intensity(&intensities[index]);
            d.draw_pixel(x as i32, y as i32, c);
        }
    }
}
