//! Generic real/complex scalar trait wrappers for BLAS and LAPACK routines

pub trait Scalar : Copy{
    type Real;
}

impl Scalar for f32{
    type Real = f32;
}

impl Scalar for f64{
    type Real = f64;
}

impl Scalar for c32{
    type Real = f32;
}

impl Scalar for c64{
    type Real = f64;
}


//#[cfg(not(feature = "simba"))]
//pub use alga::general::{RealField, ComplexField};
//#[cfg(not(feature = "simba"))]
//pub use alga::general::{SubsetOf, SupersetOf};


//#[cfg(feature = "simba")]
//pub use simba::scalar::{RealField, ComplexField};
//#[cfg(feature = "simba")]
//pub use simba::scalar::{SupersetOf, SubsetOf};

pub use cblas::Layout as Layout;
pub use cblas::Transpose as Transpose;

pub use num_complex::Complex32 as c32;
pub use num_complex::Complex64 as c64;


pub mod blas;
pub mod lapack;

pub use blas::*;
pub use lapack::*;

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use openblas_src;

    #[test]
    fn link() {
        ()
    }
}