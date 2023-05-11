mod libs;
use fractal::Intensity;
use raylib::{color::Color, prelude::*};

fn main() {
    let (mut rl, thrd) = raylib::init().size(800, 600).title("Fractos").build();

    let mut intensities : Vec::<Intensity> = vec![];
    for x in 0..600 {
        for _ in 0..800 {
        let divergence = x as f32 / 300.;
        intensities.push(Intensity{ module: 16., divergence })
      }
    };

    loop {
    let mut d = rl.begin_drawing(&thrd);
    d.clear_background(Color::WHITE);
    libs::render::render(
        800,
        600,
        &intensities,
        &libs::render::Grayscale,
        &mut d,
    );
    }
}
