use clap::Parser;
use complex;
use std::str::FromStr;

fn parse_c(arg: &str) -> Result<complex::Complex, std::io::Error> {
    let arg = arg.trim_start_matches("(").trim_end_matches(")");
    let mut cs = arg.split(",");
    let Some(re) = cs.next() else { return Err(std::io::Error::new(std::io::ErrorKind::Other, "not a complex")) };
    let Ok (re) = f64::from_str(re) else { return Err(std::io::Error::new(std::io::ErrorKind::Other, "not a complex")) };
    let Some(im) = cs.next() else { return Err(std::io::Error::new(std::io::ErrorKind::Other, "not a complex")) };
    let Ok (im) = f64::from_str(im) else { return Err(std::io::Error::new(std::io::ErrorKind::Other, "not a complex")) };
    let None = cs.next() else { return Err(std::io::Error::new(std::io::ErrorKind::Other, "not a complex")) };
    return Ok (complex::Complex::new(re, im))
}

fn parse_r(arg: &str) -> Result<WindowResolution, std::io::Error> {
    let mut cs = arg.split("x");
    let Some(xres) = cs.next() else { return Err(std::io::Error::new(std::io::ErrorKind::Other, "not a complex")) };
    let Ok (xres) = u32::from_str(xres) else { return Err(std::io::Error::new(std::io::ErrorKind::Other, "not a complex")) };
    let xres = xres - (xres % 10);
    let Some(yres) = cs.next() else { return Err(std::io::Error::new(std::io::ErrorKind::Other, "not a complex")) };
    let Ok (yres) = u32::from_str(yres) else { return Err(std::io::Error::new(std::io::ErrorKind::Other, "not a complex")) };
    let yres = yres - (yres % 10);
    let None = cs.next() else { return Err(std::io::Error::new(std::io::ErrorKind::Other, "not a complex")) };
    return Ok (WindowResolution{xres, yres})
}

#[derive(Copy,Clone)]
pub struct WindowResolution { pub xres: u32, pub yres: u32 }

impl std::fmt::Display for WindowResolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_fmt(format_args!("{}x{}",self.xres, self.yres))
    }
}

const DEFAULT_JULIA_CONSTANT : complex::Complex = complex::Complex{re: 0.3, im: 0.5};
const DEFAULT_WINDOW_RESOLUTION : WindowResolution = WindowResolution{xres: 800, yres: 600};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = DEFAULT_JULIA_CONSTANT, value_parser = parse_c)]
    pub julia_constant: complex::Complex,

    #[arg(short, long, default_value_t = 50)]
    pub max_iter: u16,

    #[arg(short, long, default_value_t = DEFAULT_WINDOW_RESOLUTION, value_parser = parse_r)]
    pub resolution: WindowResolution,
}
