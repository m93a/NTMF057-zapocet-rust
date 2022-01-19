use num_traits::{Num, FromPrimitive};
use std::ops::Range;

pub struct NumRangeIterator<'a, T> {
    iteree: &'a Range<T>,
    step_count: usize,
    current_index: usize,
}

impl<'a, T: Num + FromPrimitive + Copy> Iterator for NumRangeIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let i = self.current_index;
        if i >= self.step_count { return None }

        self.current_index += 1;

        let conv = |x: usize| T::from_usize(x).expect("Cannot convert from usize.");

        let len = self.iteree.end - self.iteree.start;
        let frac = conv(i) / conv(self.step_count);

        Some(len * frac)
    }
}

pub trait NumRangeIterable<T> {
    fn iter(&self, step_count: usize) -> NumRangeIterator<T>;
}

impl<T: Num + FromPrimitive + Copy> NumRangeIterable<T> for Range<T> {
    fn iter(&self, step_count: usize) -> NumRangeIterator<T> {
        NumRangeIterator {
            iteree: self,
            step_count,
            current_index: 0,
        }
    }
}

macro_rules! or_continue {
    ($res:expr) => {
        match $res {
            Some(val) => val,
            None => continue
        }
    };
}

pub fn ok<E>() -> Result<(), E> { Ok(()) }
