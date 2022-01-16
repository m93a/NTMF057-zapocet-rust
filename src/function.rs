use num_traits::{Num, FromPrimitive};

pub type Function<T, const IN_DIM: usize, const OUT_DIM: usize> = dyn Fn([T; IN_DIM]) -> [T; OUT_DIM];


pub struct SampledFunctionRegular1D<T: Num, const COUNT: usize> {
    pub data: [T; COUNT],
    pub start: T,
    pub step: T
}

fn from_usize<T: FromPrimitive>(n: usize) -> T {
    T::from_usize(n).expect(
        "The number type cannot be multiplied by usize. \
         For non-standard number types choose an irregular grid."
    )
}

impl<T: Num + FromPrimitive + Copy, const COUNT: usize> SampledFunctionRegular1D<T, COUNT> {

    pub fn end(&self) -> T {
        self.start + self.step * from_usize(COUNT - 1)
    }

    pub fn get_x(&self, i: usize) -> T {
        self.start + self.step * from_usize(i)
    }

    pub fn from_function(f: &Function<T, 1, 1>, start: T, step: T) -> SampledFunctionRegular1D<T, COUNT> {
        let mut data: [T; COUNT] = [T::zero(); COUNT];

        for i in 1..COUNT {
            let x = start + step * from_usize(i);
            data[i] = f([x])[0];
        }

        SampledFunctionRegular1D { data, start, step }
    }
}

