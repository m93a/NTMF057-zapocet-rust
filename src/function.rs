use num_traits::{Num, FromPrimitive};

pub type Function<T, const IN_DIM: usize, const OUT_DIM: usize> = dyn Fn([T; IN_DIM]) -> [T; OUT_DIM];


pub struct SampledFunctionRegular1D<T: Num, const COUNT: usize> {
    pub data: [T; COUNT],
    pub start: T,
    pub end: T,
    step: T
}

impl<T: Num + FromPrimitive + Copy, const COUNT: usize> SampledFunctionRegular1D<T, COUNT> {
    pub fn new(start: T, end: T, data: [T; COUNT]) -> Self {
        let mut res = SampledFunctionRegular1D { data, start, end, step: T::zero() };
        res.step = res.length() / from_usize(COUNT);
        res
    }

    pub fn count(&self) -> usize { COUNT }
    pub fn step(&self) -> T { self.step }
    pub fn length(&self) -> T { self.end - self.start }

    pub fn get_x(&self, i: usize) -> T {
        self.start + self.length() * from_f32(i as f32 / COUNT as f32)
    }

    pub fn from_function(f: &Function<T, 1, 1>, start: T, end: T) -> SampledFunctionRegular1D<T, COUNT> {
        let mut res = SampledFunctionRegular1D::new(start, end, [T::zero(); COUNT]);

        for i in 1..COUNT {
            let x = res.get_x(i);
            res.data[i] = f([x])[0];
        }

        res
    }
}

pub struct SampledFunctionRegular1DIterator<T: Num, const COUNT: usize> {
    iteree: SampledFunctionRegular1D<T, COUNT>,
    index: usize
}

pub struct SampledFunctionRegular1DIteratorRef<'a, T: Num, const COUNT: usize> {
    iteree: &'a SampledFunctionRegular1D<T, COUNT>,
    index: usize
}



impl<T: Num + FromPrimitive + Copy, const COUNT: usize>
    Iterator for SampledFunctionRegular1DIterator<T, COUNT>
{
    type Item = (T, T);

    fn next(&mut self) -> Option<(T,T)> {
        let i = self.index;
        self.index += 1;

        if i >= COUNT { None }
        else { Some((self.iteree.get_x(i), self.iteree.data[i])) }
    }
}

impl<T: Num + FromPrimitive + Copy, const COUNT: usize>
    IntoIterator for SampledFunctionRegular1D<T, COUNT>
{
    type Item = (T, T);
    type IntoIter = SampledFunctionRegular1DIterator<T, COUNT>;

    fn into_iter(self) -> SampledFunctionRegular1DIterator<T, COUNT> {
        SampledFunctionRegular1DIterator { iteree: self, index: 0 }
    }
}


fn from_usize<T: FromPrimitive>(n: usize) -> T {
    T::from_usize(n).expect(
        "The number type cannot be multiplied by usize. \
         For non-standard number types choose an irregular grid."
    )
}

fn from_f32<T: FromPrimitive>(x: f32) -> T {
    T::from_f32(x).expect(
        "The number type cannot be multiplied by f32. \
         For non-standard number types choose an irregular grid."
    )
}
