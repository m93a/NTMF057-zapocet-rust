use num_traits::{Num, FromPrimitive, ToPrimitive, NumCast};
use array_macro::array;

pub type Function<X, Y, const IN_DIM: usize, const OUT_DIM: usize> = dyn Fn([X; IN_DIM]) -> [Y; OUT_DIM];

pub enum Interpolation {
    Linear
}

use Interpolation::Linear;




pub struct SampledFunctionRegular1D<X, Y, const FN_LEN: usize, const ARR_LEN: usize>
where X: Num + Copy + FromPrimitive + ToPrimitive, Y: Num + Copy + NumCast
{
    pub data: [Y; ARR_LEN],
    pub start: X,
    pub end: X,
    step: X
}

pub fn get_x<X: Num + FromPrimitive + Copy>(start: X, end: X, sample_count: usize, index: usize) -> X {
    start + (end - start) * from_f32(index as f32 / sample_count as f32)
}

impl<X, Y, const LEN: usize> SampledFunctionRegular1D<X, Y, LEN, LEN>
where X: Num + Copy + FromPrimitive + ToPrimitive, Y: Num + Copy + NumCast
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


impl<X, Y, const FN_LEN: usize, const ARR_LEN: usize> SampledFunctionRegular1D<X, Y, FN_LEN, ARR_LEN>
where X: Num + Copy + FromPrimitive + ToPrimitive, Y: Num + Copy + NumCast
{
    pub fn sample_count(&self) -> usize { FN_LEN }
    pub fn step(&self) -> X { self.step }

    pub fn length(&self) -> X where X: Num { self.end - self.start }

    pub fn get_x(&self, i: usize) -> X
    where X: Num + FromPrimitive {
        self.start + self.length() * from_f32(i as f32 / FN_LEN as f32)
    }

    pub fn last_x(&self) -> X { self.get_x(FN_LEN - 1) }

    pub fn iter<'a>(&'a self) -> SampledFunctionRegular1DIterator<'a, X, Y, FN_LEN, ARR_LEN>
    {
        SampledFunctionRegular1DIterator { iteree: self, index: 0 }
    }

    pub fn get_index_from_x(&self, x: X) -> Option<usize> {
        let len = self.end - self.start;
        let normalized_x = x/len;
        let approx_index = normalized_x * from_usize(FN_LEN);
        let index = to_f32( approx_index ).floor();
        if index < 0. { None } else { Some(index as usize) }
    }

    pub fn interpolate(&self, mode: Interpolation) -> impl Fn(X) -> Y + '_
    {
        match mode {
            Linear => move |x: X| self.interpolate_y_linear(x)
        }
    }

    fn interpolate_y_linear (&self, x: X) -> Y
    {
        let conv = |x: X| { Y::from(x).expect("Cannot convert type X to type Y.")  };

        let i = match self.get_index_from_x(x) {
            None => 0,
            Some(i) if i < FN_LEN-1 => i,
            _ => FN_LEN-2,
        };

        let x1 = self.get_x(i);
        let x2 = self.get_x(i+1);
        let y1 = self.data[i];
        let y2 = self.data[i+1];

        let slope = (y2 - y1)/conv(x2 - x1);
        let offset = y1 - slope * conv(x1);

        return slope * conv(x) + offset;
    }
}




pub struct SampledFunctionRegular1DIterator<'a, X, Y, const FN_LEN: usize, const ARR_LEN: usize>
where X: Num + Copy + FromPrimitive + ToPrimitive, Y: Num + Copy + NumCast
{
    iteree: &'a SampledFunctionRegular1D<X, Y, FN_LEN, ARR_LEN>,
    index: usize
}


impl<'a, X, Y, const FN_LEN: usize, const ARR_LEN: usize> Iterator
for SampledFunctionRegular1DIterator<'a, X, Y, FN_LEN, ARR_LEN>
where X: Num + Copy + FromPrimitive + ToPrimitive, Y: Num + Copy + NumCast
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
