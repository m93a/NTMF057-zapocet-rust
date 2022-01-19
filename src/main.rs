#![allow(dead_code)]

use std::f32::consts::PI;

mod function;
use function::{SampledFunctionRegular1D};

mod integral;


fn sin([x]: [f32; 1]) -> [f32; 1] {[ x.sin() ]}

fn main() {
    let ssin = SampledFunctionRegular1D::<f32, 50>::from_function(&sin, 0., 2.*PI);

    println!("Start: {}, End: {}, Step: {}", ssin.start, ssin.end, ssin.step());

    for (x, y) in &ssin {
        println!("Value: x = {:.2}, y = {:.4}", x, y);
    }

    println!("Hello, world! {}", ssin.data[0]);
}
