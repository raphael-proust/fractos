use complex::Complex;
use fractal::{Algo, Fractal, Intensity};
use messages::{Answer, Task};

use rayon::prelude::*;

fn complex_of_pos(index: u32, task: &Task, y_res: u32) -> Complex {
    let f_index: f64 = index.into();
    let r_delta = task.range.max().x() - task.range.min().x();
    let i_delta = task.range.max().y() - task.range.min().y();
    let x_res = <u32 as Into<f64>>::into(task.resolution.x.get());
    let i_factor: f64 = f_index / x_res;
    let y_res = <u32 as Into<f64>>::into(y_res);
    let r_factor: f64 = f_index % x_res;
    Complex::new(
        task.range.min().x() + r_delta / x_res * r_factor,
        task.range.min().y() + i_delta / y_res * i_factor,
    )
}

pub(crate) fn seq_handle_task(task: &Task) -> Answer {
    let y_res = task.resolution.y.get();
    let size: u32 = task.resolution.x.get() * task.resolution.y.get();
    let Algo::Julia(algo) = &task.algo;
    let res: Vec<Intensity> = (0..size)
        .into_iter()
        .map(|index| {
            let c = complex_of_pos(index, task, y_res);
            algo.eval(task.itermax.into(), c)
        })
        .collect();
    Answer { matrix: res }
}

pub(crate) fn par_handle_task(task: &Task) -> Answer {
    let y_res = task.resolution.y.get();
    let size: u32 = task.resolution.x.get() * task.resolution.y.get();
    let Algo::Julia(algo) = &task.algo;
    let res: Vec<fractal::Intensity> = (0..size)
        .into_par_iter()
        .map(|index| {
            let c = complex_of_pos(index, task, y_res);
            algo.eval(task.itermax.into(), c)
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
    use fractal::{Algo, Julia};
    use messages::{Answer, Task};

    #[test]
    fn it_works() {
        let j = Julia {
            c: Complex::new(0., 0.),
            divergence_threshold_square: 16.,
        };
        let task = Task::new(Algo::Julia(j), 800, 600, -1.0, -1.0, 1.0, 1.0, 100);
        let Answer { matrix: par_result } = par_handle_task(&task);
        let Answer { matrix: seq_result } = seq_handle_task(&task);

        assert!(seq_result.iter().eq(par_result.iter()));
    }
}
