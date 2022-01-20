use crate::utils::GeneratorIterable;
use num_traits::Num;
use std::iter::once;



/// Yields positive integers.
///
/// 1, 2, 3, 4, 5, ...
pub fn naturals<T: Num + Copy>() -> impl Iterator<Item=T> {
    (move || {
        let t1 = T::one();
        let mut n = t1;

        loop {
            yield n;
            n = n + t1;
        }
    }).iter()
}

/// Yields alternating positive and negative one.
/// 
/// 1, -1, 1, -1, 1, ...
pub fn alternating<T: Num + Copy>() -> impl Iterator<Item=T> {
    (move || {
        let plus1 = T::one();
        let minus1 = T::zero() - plus1;

        loop {
            yield plus1;
            yield minus1;
        }
    }).iter()
}


/// Yields factorials, starting with 0! = 1.
/// 
/// 1, 1, 2, 6, 24, ...
pub fn factorial<T: Num + Copy>() -> impl Iterator<Item=T> {
    (move || {
        let t1 = T::one();

        let mut k = t1;
        let mut prod = k;

        loop {
            yield prod;
            prod = prod * k;
            k = k + t1;
        }
    }).iter()
}

/// Yields the Taylor series coefficients of exp(x)
/// 
/// 1, 1, 1/2, 1/6, 1/24, ...
pub fn t_exp<T: Num + Copy>() -> impl Iterator<Item=T> {
    (move || {
        let t1 = T::one();
        for x in factorial::<T>() { yield t1 / x; }
    }).iter()
}

/// Yields the nonzero coefficients of the Taylor series of cosh(x)
/// 
/// 1, 1/2, 1/24, 1/720, ...
pub fn nonzero_t_cosh<T: Num + Copy>() -> impl Iterator<Item=T> {
    t_exp::<T>().step_by(2)
}

/// Yields the Taylor series coefficients of cosh(x)
/// 
/// 1, 0, 1/2, 0, 1/24, 0, 1/720, ...
pub fn t_cosh<T: Num + Copy>() -> impl Iterator<Item=T> {
    nonzero_t_cosh::<T>().intersperse(T::zero())
}

/// Yields the nonzero coefficients of the Taylor series of cos(x)
/// 
/// 1, -1/2, 1/24, -1/720, ...
pub fn nonzero_t_cos<T: Num + Copy>() -> impl Iterator<Item=T> {
    (move || {
        for (x, sign) in nonzero_t_cosh::<T>().zip(alternating::<T>()) { yield sign * x }
    }).iter()
}

/// Yields the Taylor series coefficients of cos(x)
/// 
/// 1, 0, -1/2, 0, 1/24, 0, -1/720, ...
pub fn t_cos<T: Num + Copy>() -> impl Iterator<Item=T> {
    nonzero_t_cos::<T>().intersperse(T::zero())
}

/// Yields the nonzero coefficients of the Taylor series of sinh(x)
/// 
/// 1, 1/6, 1/120, 1/5040, ...
pub fn nonzero_t_sinh<T: Num + Copy>() -> impl Iterator<Item=T> {
    t_exp::<T>().skip(1).step_by(2)
}

/// Yields the Taylor series coefficients of sinh(x)
/// 
/// 0, 1, 0, 1/6, 0, 1/120, ...
pub fn t_sinh<T: Num + Copy>() -> impl Iterator<Item=T> {
    let t0 = T::zero();
    once(t0).chain(nonzero_t_sinh::<T>().intersperse(t0))
}

/// Yields the nonzero coefficients of the Taylor series of sin(x)
/// 
/// 1, -1/6, 1/120, -1/5040, ...
pub fn nonzero_t_sin<T: Num + Copy>() -> impl Iterator<Item=T> {
    (move || {
        for (x, sign) in nonzero_t_sinh::<T>().zip(alternating::<T>()) { yield sign * x }
    }).iter()
}

/// Yields the Taylor series coefficients of sin(x)
/// 
/// 0, 1, 0, -1/6, 0, 1/120, ...
pub fn t_sin<T: Num + Copy>() -> impl Iterator<Item=T> {
    let t0 = T::zero();
    once(t0).chain(nonzero_t_sin::<T>().intersperse(t0))
}

/// Yields the Taylor series coefficients of ln(1+x)
/// 
/// 0, 1, -1/2, 1/3, -1/4, 1/5, ...
pub fn t_ln1p<T: Num + Copy>() -> impl Iterator<Item=T> {
    (move || {
        yield T::zero(); // no constant term
        for (n, sign) in naturals::<T>().zip(alternating::<T>()) { yield sign / n; }
    }).iter()
}

