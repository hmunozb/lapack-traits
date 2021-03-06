use crate::Scalar;
use num_complex::Complex32 as c32;
use num_complex::Complex64 as c64;

pub trait ITamax: Scalar {
    unsafe fn amax(n: i32, x: &[Self], incx: i32) -> i32;
}

macro_rules! impl_itamax(
    ($N: ty, $itamax: path) => (
        impl ITamax for $N{
            unsafe fn amax(n: i32, x: &[Self], incx: i32) -> i32{
                $itamax(n, x, incx)
            }
        }
    )
);

impl_itamax!(f32, cblas::isamax);
impl_itamax!(f64, cblas::idamax);
impl_itamax!(c32, cblas::icamax);
impl_itamax!(c64, cblas::izamax);