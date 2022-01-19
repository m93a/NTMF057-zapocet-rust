#![allow(dead_code)]

use std::f32::consts::PI;
const PI2: f32 = 2.*PI;

use std::fs::File;
use std::io::{Error, Write};

mod function;
use function::{SampledFunctionRegular1D};
use function::Interpolation::Linear;

mod integral;

#[macro_use]
mod utils;
use utils::{NumRangeIterable, ok};


fn sin([x]: [f32; 1]) -> [f32; 1] {[ x.sin() ]}

fn main() -> Result<(), Error>{
    const PTS: usize = 10;

    let ssin = SampledFunctionRegular1D
        ::<f32, f32, PTS, PTS>
        ::from_function(&sin, 0., PI2);

    println!("Start: {}, End: {}, Step: {}", ssin.start, ssin.end, ssin.step());


    let mut file = File::create("./data1.txt")?;
    for (x, y) in ssin.iter() {
        writeln!(file, "{:.4} {:.4}", x, y)?;
    }

    
    let mut file = File::create("./data2.txt")?;
    for x in (0.0..PI2).iter(PTS*50) {
        let y = or_continue!( ssin.interpolate_y(x, Linear) );
        writeln!(file, "{:.4} {:.4}", x, y)?;
    }

    return ok();
}
