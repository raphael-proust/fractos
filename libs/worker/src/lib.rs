use complex::Complex;
use fractal::{Fractal, Intensity};
use messages::{Answer, Task};

use rayon::prelude::*;

fn complex_of_pos(index: u32, task: &Task, yy: u32) -> Complex {
    let f_index: f64 = index.into();
    let r_delta = task.range.max().x() - task.range.min().x();
    let i_delta = task.range.max().y() - task.range.min().y();
    let yy = <u32 as Into<f64>>::into(yy);
    let r_factor: f64 = f_index / yy;
    let i_factor: f64 = f_index % yy;
    Complex::new(
        task.range.min().x() + r_delta * r_factor,
        task.range.min().y() + i_delta * i_factor,
    )
}

pub(crate) fn seq_handle_task(task: &Task) -> Answer {
    let yy = task.resolution.y.get();
    let size: u32 = task.resolution.x.get() * task.resolution.y.get();
    let res: Vec<Intensity> = (0..size)
        .into_iter()
        .map(|index| {
            let c = complex_of_pos(index, task, yy);
            task.algo.eval(task.itermax.into(), c)
        })
        .collect();
    Answer { matrix: res }
}

pub(crate) fn par_handle_task(task: &Task) -> Answer {
    let yy = task.resolution.y.get();
    let size: u32 = task.resolution.x.get() * task.resolution.y.get();
    let res: Vec<fractal::Intensity> = (0..size)
        .into_par_iter()
        .map(|index| {
            let c = complex_of_pos(index, task, yy);
            task.algo.eval(task.itermax.into(), c)
        })
        .collect();
    Answer { matrix: res }
}

pub fn handle_task(task: &Task) -> Answer {
    par_handle_task(task)
}

#[cfg(test)]
mod tests {
    use crate::{par_handle_task, seq_handle_task};
    use complex::Complex;
    use fractal::Julia;
    use messages::{point::Point, range::Range, resolution::Resolution, Answer, Task};
    use std::num::{NonZeroU16, NonZeroU32};

    #[test]
    fn it_works() {
        let j = Julia {
            c: Complex::new(0., 0.),
            divergence_threshold_square: 16.,
        };
        let task = Task {
            algo: j,

            resolution: Resolution {
                x: NonZeroU32::new(800).unwrap(),
                y: NonZeroU32::new(600).unwrap(),
            },
            range: Range::new(
                Point::new(0.0, 0.0).unwrap(),
                Point::new(800., 600.).unwrap(),
            )
            .unwrap(),
            itermax: NonZeroU16::new(100).unwrap(),
        };
        let Answer { matrix: par_result } = par_handle_task(&task);
        let Answer { matrix: seq_result } = seq_handle_task(&task);

        assert!(seq_result.iter().eq(par_result.iter()));
    }
}
