#![allow(dead_code)]
#![allow(unused_macros)]
#![feature(generators, generator_trait)]

use std::f32::consts::PI;
const PI2: f32 = 2.*PI;

use std::fs::File;
use std::io::{Error, Write};

mod function;
use function::{SampledFunctionRegular1D};
use function::Interpolation::Linear;

mod integral;

mod constant_sequences;
use constant_sequences::{factorial, t_exp};

#[macro_use]
mod utils;
use utils::{NumRangeIterable, GeneratorIterable, ok};


fn sin([x]: [f32; 1]) -> [f32; 1] {[ x.sin() ]}

fn main() -> Result<(), Error>{
    const PTS: usize = 10;

    let ssin = SampledFunctionRegular1D
        ::<f32, f32, PTS, PTS>
        ::from_function(&sin, 0., PI2);

    println!("Start: {}, End: {}, Step: {}", ssin.start, ssin.end, ssin.step());

    {
        let mut file = File::create("./data1.txt")?;
        for (x, y) in ssin.iter() {
            writeln!(file, "{:.4} {:.4}", x, y)?;
        }
    }

    {
        let mut file = File::create("./data2.txt")?;
        let f = ssin.interpolate(Linear);

        for x in (0.0..PI2).iter(PTS*50) {
            writeln!(file, "{:.4} {:.4}", x, f(x))?;
        }
    }

    println!("Hello world! {}", ssin.data[0]);

    println!(); println!();
    print!("Factorials: ");
    for x in factorial::<i32>().iter().take(11) {
        print!("{}, ", x);
    }

    println!(); println!();
    print!("exp x = ");
    for (k, a) in t_exp::<f64>().iter().enumerate().take(11) {
        print!("{:.4} x^{} + ", a, k);
    }


    return ok();
}
