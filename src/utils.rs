use num_traits::{Num, FromPrimitive};
use std::ops::{Range, Generator, GeneratorState};
use std::pin::Pin;




// NumRangeIterator

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




// GeneratorIterator

pub struct GeneratorIterator<G>(Pin<Box<G>>);

impl<G> GeneratorIterator<G>
where G: Generator<Return = ()>
{
    pub fn new(gen: G) -> Self {
        Self(Box::pin(gen))
    }
}

impl<G> Iterator for GeneratorIterator<G>
where G: Generator<Return = ()>
{
    type Item = G::Yield;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.as_mut().resume(()) {
            GeneratorState::Yielded(x) => Some(x),
            GeneratorState::Complete(_) => None,
        }
    }
}

pub trait GeneratorIterable<G>
where G: Generator<Return = ()>
{
    fn iter(self) -> GeneratorIterator<G>;
}

impl<G> GeneratorIterable<G> for G
where G: Generator<Return = ()> {
    fn iter(self) -> GeneratorIterator<G> {
        GeneratorIterator::new(self)
    }
}




// or_continue!

macro_rules! or_continue {
    ($res:expr) => {
        match $res {
            Some(val) => val,
            None => continue
        }
    };
}

pub fn ok<E>() -> Result<(), E> { Ok(()) }
