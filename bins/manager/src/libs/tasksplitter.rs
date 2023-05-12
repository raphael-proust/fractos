use messages::Task;
use fractal;

pub fn split_quad(
    iter_max: u16,
    fractal: &impl fractal::Fractal,
    xblocks: u32,
    yblocks: u32,
    block_size: u32,
    quad_exp: u8,
) -> Vec<(Task, u32, u32)> {
    let quad_fact = 1 << quad_exp;
    assert!(xblocks % quad_fact == 0);
    let xblocks = xblocks / quad_fact;
    assert!(xblocks % quad_fact == 0);
    let yblocks = yblocks / quad_fact;
    assert!(yblocks % quad_fact == 0);

    let x_task_delta = 2. / quad_fact as f64;
    let y_task_delta = 2. / quad_fact as f64;

    let mut v = vec![];

    for xindex in 0..quad_fact {
        for yindex in 0..quad_fact {
            let algo = fractal.into_algo();
            let re_start = -1.0 + (x_task_delta * xindex as f64) + (2. / xblocks as f64);
            let re_end = re_start + x_task_delta;
            let im_start = -1.0 + (y_task_delta * yindex as f64) + (2. / yblocks as f64);
            let im_end = im_start + y_task_delta;
            let task = Task::new(algo, xblocks, yblocks, re_start, im_start, re_end, im_end, iter_max);
            v.push((task, xindex, yindex))
        }
    };

    v

}

