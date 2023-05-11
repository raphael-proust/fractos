use fractal::Intensity;

use raylib::{color::rcolor, color::Color, prelude::*};

pub trait ColorMap {
    fn of_intensity(&self, intensity: &Intensity) -> Color;
}

fn boost(x: f32) -> f32 {
    1.0 - x.powf(0.2)
}

fn normalize_u8(x: f32) -> u8 {
    let x = x * 255.;
    let x = x as i64;
    let x = x.clamp(0, 255);
    let x = x as u8;
    x
}

pub struct Grayscale;

impl ColorMap for Grayscale {
    fn of_intensity(&self, intensity: &Intensity) -> Color {
        let Intensity {
            divergence,
            module: _,
        } = intensity;
        let div = boost(*divergence);
        let div = normalize_u8(div);
        let color = rcolor(div, div, div, 255);
        color
    }
}

pub struct Fire;

impl ColorMap for Fire {
    fn of_intensity(&self, intensity: &Intensity) -> Color {
        let Intensity {
            divergence,
            module: _,
        } = intensity;
        let div = boost(*divergence);
        let div = normalize_u8(div);
        let color = rcolor(div, div / 2, div / 4, 255);
        color
    }
}

pub fn render(
    xres: u32,
    yres: u32,
    intensities: &Vec<Intensity>,
    color_map: &impl ColorMap,
    d: &mut RaylibDrawHandle,
) -> () {
    for y in 0..yres {
        for x in 0..xres {
            let index: usize = (x + y * xres) as usize;
            let c = color_map.of_intensity(&intensities[index]);
            d.draw_pixel(x as i32, y as i32, c);
        }
    }
}
