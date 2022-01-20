
use std::ops::{Generator};
use crate::utils::GeneratorIterable;
use num_traits::Num;



pub fn factorial<T: Num + Copy>() -> impl Generator<Yield=T, Return=()> {
    move || {
        let t1 = T::one();

        let mut k = t1;
        let mut prod = k;

        loop {
            yield prod;
            prod = prod * k;
            k = k + t1;
        }
    }
}

pub fn t_exp<T: Num + Copy>() -> impl Generator<Yield=T, Return=()> {
    move || {
        let t1 = T::one();

        for kfac in factorial::<T>().iter() {
            yield t1 / kfac;
        }
    }
}



