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
        let color = rcolor(div, div, div, div);
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

pub fn render_averaged_chunk(
    xoffs: u32,                   // screen-space coordinate
    yoffs: u32,                   // screen-space coordinate
    xsize: u32,                   // screen-space coordinate
    ysize: u32,                   // screen-space coordinate
    block_size: u32,              // screen-space size of a block
    intensities: &Vec<Intensity>, // row-major averaged-space indexing
    color_map: &impl ColorMap,
    d: &mut RaylibDrawHandle,
) -> () {
    assert!(xsize % block_size == 0);
    assert!(ysize % block_size == 0);
    let xblocks = xsize / block_size;
    let yblocks = ysize / block_size;
    for yb in 0..yblocks {
        for xb in 0..xblocks {
            let index: usize = (xb + yb * xblocks) as usize;
            let c = color_map.of_intensity(&intensities[index]);
            let xstart = xoffs + xb * block_size;
            let ystart = yoffs + yb * block_size;
            for y in ystart..ystart + block_size {
                for x in xstart..xstart + block_size {
                    d.draw_pixel(x as i32, y as i32, c);
                }
            }
        }
    }
}

pub fn render_chunk(
    xoffs: u32,
    yoffs: u32,
    xsize: u32,
    ysize: u32,
    intensities: &Vec<Intensity>,
    color_map: &impl ColorMap,
    d: &mut RaylibDrawHandle,
) -> () {
    for y in yoffs..yoffs + ysize {
        for x in xoffs..xoffs + xsize {
            let index: usize = (x + y * xsize) as usize;
            let c = color_map.of_intensity(&intensities[index]);
            d.draw_pixel(x as i32, y as i32, c);
        }
    }
}

pub fn render(
    xres: u32,
    yres: u32,
    intensities: &Vec<Intensity>,
    color_map: &impl ColorMap,
    d: &mut RaylibDrawHandle,
) -> () {
    render_chunk(0, 0, xres, yres, intensities, color_map, d);
}
