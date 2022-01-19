use num_traits::{Num, FromPrimitive, ToPrimitive, NumCast};
use array_macro::array;

pub type Function<X, Y, const IN_DIM: usize, const OUT_DIM: usize> = dyn Fn([X; IN_DIM]) -> [Y; OUT_DIM];

pub enum Interpolation {
    Linear
}

use Interpolation::Linear;


pub struct SampledFunctionRegular1D<X, Y, const FN_LEN: usize, const ARR_LEN: usize> {
    pub data: [Y; ARR_LEN],
    pub start: X,
    pub end: X,
    step: X
}

pub fn get_x<X: Num + FromPrimitive + Copy>(start: X, end: X, sample_count: usize, index: usize) -> X {
    start + (end - start) * from_f32(index as f32 / sample_count as f32)
}

impl<X, Y, const LEN: usize> SampledFunctionRegular1D<X, Y, LEN, LEN>
where X: Num + FromPrimitive + Copy, Y: Copy
{
    pub fn new(start: X, end: X, data: [Y; LEN]) -> Self {
        let mut res = SampledFunctionRegular1D { data, start, end, step: X::zero() };
        res.step = res.length() / from_usize(LEN);
        res
    }

    pub fn from_function(f: &Function<X, Y, 1, 1>, start: X, end: X) -> SampledFunctionRegular1D<X, Y, LEN, LEN>
    {
        let data = array![
            i => {
                let x = get_x(start, end, LEN, i);
                f([x])[0]
            };
            LEN
        ];

        SampledFunctionRegular1D::new(start, end, data)
    }
}

impl<X: Copy, Y, const FN_LEN: usize, const ARR_LEN: usize> SampledFunctionRegular1D<X, Y, FN_LEN, ARR_LEN>
{
    pub fn sample_count(&self) -> usize { FN_LEN }
    pub fn step(&self) -> X { self.step }

    pub fn length(&self) -> X where X: Num { self.end - self.start }
}

impl<X, Y, const FN_LEN: usize, const ARR_LEN: usize> SampledFunctionRegular1D<X, Y, FN_LEN, ARR_LEN>
where X: Num + Copy + FromPrimitive
{
    pub fn get_x(&self, i: usize) -> X
    where X: Num + FromPrimitive {
        self.start + self.length() * from_f32(i as f32 / FN_LEN as f32)
    }

    pub fn last_x(&self) -> X { self.get_x(FN_LEN - 1) }

    pub fn iter<'a>(&'a self) -> SampledFunctionRegular1DIterator<'a, X, Y, FN_LEN, ARR_LEN>
    where Y: Copy {
        self.into_iter()
    }
}

impl<X, Y, const FN_LEN: usize, const ARR_LEN: usize> SampledFunctionRegular1D<X, Y, FN_LEN, ARR_LEN>
where X: Num + Copy + FromPrimitive + ToPrimitive
{

    pub fn get_index_from_x(&self, x: X) -> Option<usize> {
        let len = self.end - self.start;
        let normalized_x = x/len;
        let approx_index = normalized_x * from_usize(FN_LEN);
        let index = to_f32( approx_index ).floor();
        if index < 0. { None } else { Some(index as usize) }
    }

    pub fn interpolate_y(&self, x: X, mode: Interpolation) -> Option<Y>
    where Y: Num + Copy + NumCast {
        let conv = |x: X| { Y::from(x).expect("Cannot convert type X to type Y.")  };

        match mode {
            Linear => {
                let i = match self.get_index_from_x(x) {
                    Some(i) if i+1 < FN_LEN => i,
                    _ => { return None; }
                };

                let x1 = self.get_x(i);
                let x2 = self.get_x(i+1);
                let y1 = self.data[i];
                let y2 = self.data[i+1];

                let slope = (y2 - y1)/conv(x2 - x1);
                let offset = y1 - slope * conv(x1);

                return Some(slope * conv(x) + offset);
            }
        };
    }
}


pub struct SampledFunctionRegular1DIterator<'a, X, Y, const FN_LEN: usize, const ARR_LEN: usize> {
    iteree: &'a SampledFunctionRegular1D<X, Y, FN_LEN, ARR_LEN>,
    index: usize
}


impl<'a, X, Y, const FN_LEN: usize, const ARR_LEN: usize> Iterator
for SampledFunctionRegular1DIterator<'a, X, Y, FN_LEN, ARR_LEN>
where X: Num + Copy + FromPrimitive, Y: Copy
{
    type Item = (X, Y);

    fn next(&mut self) -> Option<(X,Y)> {
        if FN_LEN < ARR_LEN { panic!("The sample count is smaller than the size of the array!"); }

        let i = self.index;
        self.index += 1;

        if i >= FN_LEN { None }
        else { Some((self.iteree.get_x(i), self.iteree.data[i])) }
    }
}


impl<'a, X, Y, const FN_LEN: usize, const ARR_LEN: usize> IntoIterator
for &'a SampledFunctionRegular1D<X, Y, FN_LEN, ARR_LEN>
where X: Num + Copy + FromPrimitive, Y: Copy
{
    type Item = (X, Y);
    type IntoIter = SampledFunctionRegular1DIterator<'a, X, Y, FN_LEN, ARR_LEN>;

    fn into_iter(self) -> SampledFunctionRegular1DIterator<'a, X, Y, FN_LEN, ARR_LEN> {
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

fn to_f32_ref<T: ToPrimitive>(x: &T) -> f32 {
    T::to_f32(x).expect("The number cannot be converted to f32.")
}

fn to_f32<T: ToPrimitive>(x: T) -> f32 {
    T::to_f32(&x).expect("The number cannot be converted to f32.")
}
