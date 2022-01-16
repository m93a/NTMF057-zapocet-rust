#![allow(dead_code)]

mod function;
use function::{SampledFunctionRegular1D};

mod integral;


fn sin([x]: [f32; 1]) -> [f32; 1] {[ x.sin() ]}

fn main() {
    let ssin = SampledFunctionRegular1D::<f32, 100>::from_function(&sin, -10.0, 0.01);

    println!("Values: {} {} {} {} {}", ssin.data[0], ssin.data[1], ssin.data[2], ssin.data[3], ssin.data[4]);
    println!("Hello, world!");
}
