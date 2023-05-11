mod libs;
use fractal::Intensity;
use raylib::{color::Color, prelude::*};

fn main() {
    let (mut rl, thrd) = raylib::init().size(800, 600).title("Fractos").build();

    let mut d = rl.begin_drawing(&thrd);
    d.clear_background(Color::WHITE);
    libs::render::render(
        0,
        0,
        &Vec::<Intensity>::new(),
        &libs::render::Grayscale,
        &mut d,
    )
}
