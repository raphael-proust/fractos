use fractal::*;
use messages::*;
use complex::Complex;

pub fn task0 () -> Task  {
    todo!()
}

pub fn handle_task (task : Task) -> Answer {
    let yy = task.resolution.y.get();
    let size : u32 = task.resolution.x.get() * task.resolution.y.get() ;
    let res : Vec<fractal::Intensity> = (0 .. size).into_iter().map(|index|{
	let f_index : f64 = index.into();
	let r_delta = task.range.max().x() - task.range.min().x();  
	let i_delta = task.range.max().y() - task.range.min().y();  
	let yy = <u32 as Into<f64>>::into(yy);
	let r_factor : f64 = f_index / yy;
	let i_factor : f64 = f_index % yy;
	let c = Complex::new(task.range.min().x() + r_delta * r_factor,
			    task.range.min().y() + i_delta * i_factor); 
	task.algo.eval(task.itermax.into(),c)
    }).collect();
    Answer {matrix : res}
}
    
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
