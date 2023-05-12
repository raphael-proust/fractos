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

const DEFAULT_JULIA_CONSTANT : complex::Complex = complex::Complex{re: 0.3, im: 0.5};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = DEFAULT_JULIA_CONSTANT, value_parser = parse_c)]
    pub julia_constant: complex::Complex,

    #[arg(short, long, default_value_t = 50)]
    pub max_iter: u16,
}
