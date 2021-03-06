use crate::Scalar;
use cblas::{Layout, Part, Transpose, ssyrk, dsyrk, csyrk, zsyrk};
use num_complex::Complex32 as c32;
use num_complex::Complex64 as c64;

pub trait Tsyrk : Scalar {
    unsafe fn syrk(layout: Layout,
            uplo: Part,
            trans: Transpose,
            n: i32,
            k: i32,
            alpha: Self,
            a: &[Self],
            lda: i32,
            beta: Self,
            c: &mut [Self],
            ldc: i32,
    );
}

macro_rules! impl_tsyrk{
    ($N: ty, $tsyrk: path) => (
        impl Tsyrk for $N{
            unsafe fn syrk(
                layout: Layout,
                uplo: Part,
                trans: Transpose,
                n: i32,
                k: i32,
                alpha: Self,
                a: &[Self],
                lda: i32,
                beta: Self,
                c: &mut [Self],
                ldc: i32
            )
            {
                $tsyrk(layout, uplo, trans, n, k, alpha, a, lda, beta, c, ldc)
            }
        }
    )
}

impl_tsyrk!(f32, ssyrk);
impl_tsyrk!(f64, dsyrk);
impl_tsyrk!(c32, csyrk);
impl_tsyrk!(c64, zsyrk);