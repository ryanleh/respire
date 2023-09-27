use crate::math::gadget::{IntModDecomposition, RingElementDecomposable};
use crate::math::int_mod::IntMod;
use crate::math::int_mod_crt::IntModCRT;
use crate::math::int_mod_cyclo::IntModCyclo;
use crate::math::int_mod_cyclo_crt::IntModCycloCRT;
use crate::math::int_mod_cyclo_eval::IntModCycloEval;
use crate::math::int_mod_poly::IntModPoly;
use crate::math::matrix::Matrix;
use crate::math::rand_sampled::*;
use crate::math::ring_elem::*;
use rand::Rng;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// TODO: documentation

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C, align(64))]
pub struct IntModCycloCRTEval<
    const D: usize,
    const N1: u64,
    const N2: u64,
    const N1_INV: u64,
    const N2_INV: u64,
    const W1: u64,
    const W2: u64,
> {
    pub(in crate::math) p1: IntModCycloEval<D, N1, W1>,
    pub(in crate::math) p2: IntModCycloEval<D, N2, W2>,
}

/// Conversions

impl<
        const D: usize,
        const N1: u64,
        const N2: u64,
        const N1_INV: u64,
        const N2_INV: u64,
        const W1: u64,
        const W2: u64,
    > From<u64> for IntModCycloCRTEval<D, N1, N2, N1_INV, N2_INV, W1, W2>
{
    fn from(a: u64) -> Self {
        Self {
            p1: a.into(),
            p2: a.into(),
        }
    }
}

impl<
        const D: usize,
        const N1: u64,
        const N2: u64,
        const N1_INV: u64,
        const N2_INV: u64,
        const W1: u64,
        const W2: u64,
    > From<(IntModCycloEval<D, N1, W1>, IntModCycloEval<D, N2, W2>)>
    for IntModCycloCRTEval<D, N1, N2, N1_INV, N2_INV, W1, W2>
{
    fn from(a: (IntModCycloEval<D, N1, W1>, IntModCycloEval<D, N2, W2>)) -> Self {
        IntModCycloCRTEval { p1: a.0, p2: a.1 }
    }
}

impl<
        const D: usize,
        const N1: u64,
        const N2: u64,
        const N1_INV: u64,
        const N2_INV: u64,
        const W1: u64,
        const W2: u64,
    > From<IntModCycloCRTEval<D, N1, N2, N1_INV, N2_INV, W1, W2>>
    for IntModCycloCRT<D, N1, N2, N1_INV, N2_INV>
{
    fn from(a: IntModCycloCRTEval<D, N1, N2, N1_INV, N2_INV, W1, W2>) -> Self {
        let p1_raw: IntModCyclo<D, N1> = a.p1.into();
        let p2_raw: IntModCyclo<D, N2> = a.p2.into();
        (p1_raw, p2_raw).into()
    }
}

impl<
        const D: usize,
        const N1: u64,
        const N2: u64,
        const N1_INV: u64,
        const N2_INV: u64,
        const W1: u64,
        const W2: u64,
        const N: u64,
    > From<&IntModCyclo<D, N>> for IntModCycloCRTEval<D, N1, N2, N1_INV, N2_INV, W1, W2>
{
    fn from(a: &IntModCyclo<D, N>) -> Self {
        IntModCycloCRTEval::from(&IntModCycloCRT::from(a))
    }
}

impl<
        const D: usize,
        const N1: u64,
        const N2: u64,
        const N1_INV: u64,
        const N2_INV: u64,
        const W1: u64,
        const W2: u64,
    > From<&IntModCycloCRT<D, N1, N2, N1_INV, N2_INV>>
    for IntModCycloCRTEval<D, N1, N2, N1_INV, N2_INV, W1, W2>
{
    fn from(a: &IntModCycloCRT<D, N1, N2, N1_INV, N2_INV>) -> Self {
        let p1_eval: IntModCycloEval<D, N1, W1> = (&a.p1).into();
        let p2_eval: IntModCycloEval<D, N2, W2> = (&a.p2).into();
        (p1_eval, p2_eval).into()
    }
}

// impl<const D: usize, const N1: u64, const N2: u64, const N1_INV: u64, const N2_INV: u64, const W1: u64, const W2: u64> From<[Z_N<N>; D]> for Z_N_CycloNTT_CRT<D, N1, N2, N1_INV, N2_INV, W1, W2> {
//     fn from(coeff: [Z_N<N>; D]) -> Self {
//         Self { coeff }
//     }
// }

// impl<const D: usize, const N1: u64, const N2: u64, const N1_INV: u64, const N2_INV: u64, const W1: u64, const W2: u64> From<PolynomialZ_N<N>> for Z_N_CycloNTT_CRT<D, N1, N2, N1_INV, N2_INV, W1, W2> {
//     fn from(polynomial: PolynomialZ_N<N>) -> Self {
//         let mut coeff: [Z_N<N>; D] = [0_u64.into(); D];
//         for (i, x) in polynomial.coeff_iter().enumerate() {
//             if i / D % 2 == 0 {
//                 coeff[i % D] += x;
//             } else {
//                 coeff[i % D] -= x;
//             }
//         }
//         coeff.into()
//     }
// }

impl<
        const D: usize,
        const N1: u64,
        const N2: u64,
        const N1_INV: u64,
        const N2_INV: u64,
        const W1: u64,
        const W2: u64,
    > From<Vec<u64>> for IntModCycloCRTEval<D, N1, N2, N1_INV, N2_INV, W1, W2>
{
    fn from(coeff: Vec<u64>) -> Self {
        IntModCycloCRTEval {
            p1: IntModCycloEval::from(IntModPoly::from(coeff.clone())),
            p2: IntModCycloEval::from(IntModPoly::from(coeff)),
        }
    }
}

// impl<const D: usize, const N1: u64, const N2: u64, const N1_INV: u64, const N2_INV: u64, const W1: u64, const W2: u64> From<Vec<Z_N<N>>> for Z_N_CycloNTT_CRT<D, N1, N2, N1_INV, N2_INV, W1, W2> {
//     fn from(coeff: Vec<Z_N<N>>) -> Self {
//         Z_N_CycloNTT_CRT::from(PolynomialZ_N::from(coeff))
//     }
// }

// impl<const D: usize, const N1: u64, const N2: u64, const N1_INV: u64, const N2_INV: u64, const W1: u64, const W2: u64> TryFrom<&Z_N_CycloNTT_CRT<D, N1, N2, N1_INV, N2_INV, W1, W2>> for Z_N<N> {
//     type Error = ();

//     /// Inverse of `From<u64>`. Errors if element is not a constant.
//     fn try_from(a: &Z_N_CycloNTT_CRT<D, N1, N2, N1_INV, N2_INV, W1, W2>) -> Result<Self, Self::Error> {
//         for i in 1..D {
//             if a.coeff[i] != Z_N::zero() {
//                 return Err(());
//             }
//         }
//         Ok(a.coeff[0])
//     }
// }

// impl<const D: usize, const N1: u64, const N2: u64, const N1_INV: u64, const N2_INV: u64, const W1: u64, const W2: u64, const W: u64> From<Z_N_CycloNTT<D, N, W>>
//     for Z_N_CycloNTT_CRT<D, N1, N2, N1_INV, N2_INV, W1, W2>
// {
//     fn from(a_eval: Z_N_CycloNTT<D, N, W>) -> Self {
//         (&a_eval).into()
//     }
// }

// impl<const D: usize, const N1: u64, const N2: u64, const N1_INV: u64, const N2_INV: u64, const W1: u64, const W2: u64, const W: u64> From<&Z_N_CycloNTT<D, N, W>>
//     for Z_N_CycloNTT_CRT<D, N1, N2, N1_INV, N2_INV, W1, W2>
// {
//     fn from(a_eval: &Z_N_CycloNTT<D, N, W>) -> Self {
//         // TODO: this should be in the type, probably
//         let mut log_d = 1;
//         while (1 << log_d) < D {
//             log_d += 1;
//         }
//         assert_eq!(1 << log_d, D);

//         let root: Z_N<N> = W.into();

//         let mut coeff: [Z_N<N>; D] = [0_u64.into(); D];
//         for (i, x) in a_eval.points_iter().enumerate() {
//             coeff[i] = x.clone();
//         }

//         bit_reverse_order(&mut coeff, log_d);
//         ntt(&mut coeff, (root * root).inverse(), log_d);

//         let mut inv_root_pow: Z_N<N> = 1u64.into();
//         let inv_root = root.inverse();
//         let inv_D = Z_N::<N>::from(D as u64).inverse();
//         for i in 0..D {
//             // divide by degree
//             coeff[i] *= inv_D;
//             // negacyclic post-processing
//             coeff[i] *= inv_root_pow;
//             inv_root_pow *= inv_root;
//         }

//         return Self { coeff };
//     }
// }

/// [`RingElementRef`] implementation

impl<
        const D: usize,
        const N1: u64,
        const N2: u64,
        const N1_INV: u64,
        const N2_INV: u64,
        const W1: u64,
        const W2: u64,
    > RingElementRef<IntModCycloCRTEval<D, N1, N2, N1_INV, N2_INV, W1, W2>>
    for &IntModCycloCRTEval<D, N1, N2, N1_INV, N2_INV, W1, W2>
{
}

impl<
        const D: usize,
        const N1: u64,
        const N2: u64,
        const N1_INV: u64,
        const N2_INV: u64,
        const W1: u64,
        const W2: u64,
    > Add for &IntModCycloCRTEval<D, N1, N2, N1_INV, N2_INV, W1, W2>
{
    type Output = IntModCycloCRTEval<D, N1, N2, N1_INV, N2_INV, W1, W2>;
    fn add(self, rhs: Self) -> Self::Output {
        IntModCycloCRTEval {
            p1: &self.p1 + &rhs.p1,
            p2: &self.p2 + &rhs.p2,
        }
    }
}

impl<
        const D: usize,
        const N1: u64,
        const N2: u64,
        const N1_INV: u64,
        const N2_INV: u64,
        const W1: u64,
        const W2: u64,
    > Sub for &IntModCycloCRTEval<D, N1, N2, N1_INV, N2_INV, W1, W2>
{
    type Output = IntModCycloCRTEval<D, N1, N2, N1_INV, N2_INV, W1, W2>;
    fn sub(self, rhs: Self) -> Self::Output {
        IntModCycloCRTEval {
            p1: &self.p1 - &rhs.p1,
            p2: &self.p2 - &rhs.p2,
        }
    }
}

impl<
        const D: usize,
        const N1: u64,
        const N2: u64,
        const N1_INV: u64,
        const N2_INV: u64,
        const W1: u64,
        const W2: u64,
    > Mul for &IntModCycloCRTEval<D, N1, N2, N1_INV, N2_INV, W1, W2>
{
    type Output = IntModCycloCRTEval<D, N1, N2, N1_INV, N2_INV, W1, W2>;
    fn mul(self, rhs: Self) -> Self::Output {
        IntModCycloCRTEval {
            p1: &self.p1 * &rhs.p1,
            p2: &self.p2 * &rhs.p2,
        }
    }
}

impl<
        const D: usize,
        const N1: u64,
        const N2: u64,
        const N1_INV: u64,
        const N2_INV: u64,
        const W1: u64,
        const W2: u64,
    > Neg for &IntModCycloCRTEval<D, N1, N2, N1_INV, N2_INV, W1, W2>
{
    type Output = IntModCycloCRTEval<D, N1, N2, N1_INV, N2_INV, W1, W2>;
    fn neg(self) -> Self::Output {
        IntModCycloCRTEval {
            p1: -&self.p1,
            p2: -&self.p2,
        }
    }
}

/// [`RingElement`] implementation

impl<
        const D: usize,
        const N1: u64,
        const N2: u64,
        const N1_INV: u64,
        const N2_INV: u64,
        const W1: u64,
        const W2: u64,
    > RingElement for IntModCycloCRTEval<D, N1, N2, N1_INV, N2_INV, W1, W2>
{
    fn zero() -> IntModCycloCRTEval<D, N1, N2, N1_INV, N2_INV, W1, W2> {
        IntModCycloCRTEval {
            p1: IntModCycloEval::zero(),
            p2: IntModCycloEval::zero(),
        }
    }
    fn one() -> IntModCycloCRTEval<D, N1, N2, N1_INV, N2_INV, W1, W2> {
        IntModCycloCRTEval {
            p1: IntModCycloEval::one(),
            p2: IntModCycloEval::one(),
        }
    }

    // #[cfg(not(target_feature = "avx2"))]
    // fn add_eq_mul(&mut self, a: &Self, b: &Self) {
    //     for i in 0..D {
    //         self.p1.points[i] += a.p1.points[i] * b.p1.points[i];
    //         self.p2.points[i] += a.p2.points[i] * b.p2.points[i];
    //     }
    // }

    // #[cfg(target_feature = "avx2")]
    fn add_eq_mul(&mut self, a: &Self, b: &Self) {
        if N1 == 0 && N2 == 0 && D % 4 == 0 {
            use std::arch::x86_64::*;

            unsafe fn ptr_add_eq_mul64(
                s_ptr: *mut __m256i,
                a_ptr: *const __m256i,
                b_ptr: *const __m256i,
            ) {
                let a = _mm256_load_si256(a_ptr);
                let b = _mm256_load_si256(b_ptr);
                let s = _mm256_load_si256(s_ptr);
                // Note: this assumes the inputs are 32 bit. For now this is fine, since this is
                // only used for "fresh" elements which come directly from CRT (and the CRT moduli
                // are 32 bit).
                let prod = _mm256_mul_epu32(a, b);
                let sum_prod = _mm256_add_epi64(s, prod);
                _mm256_store_si256(s_ptr, sum_prod);
            }

            unsafe {
                for i in 0..D / 4 {
                    let a_p1_ptr =
                        a.p1.points.get_unchecked(4 * i) as *const IntMod<N1> as *const __m256i;
                    let b_p1_ptr =
                        b.p1.points.get_unchecked(4 * i) as *const IntMod<N1> as *const __m256i;
                    let self_p1_ptr =
                        self.p1.points.get_unchecked_mut(4 * i) as *mut IntMod<N1> as *mut __m256i;
                    ptr_add_eq_mul64(self_p1_ptr, a_p1_ptr, b_p1_ptr);

                    let a_p2_ptr =
                        a.p2.points.get_unchecked(4 * i) as *const IntMod<N2> as *const __m256i;
                    let b_p2_ptr =
                        b.p2.points.get_unchecked(4 * i) as *const IntMod<N2> as *const __m256i;
                    let self_p2_ptr =
                        self.p2.points.get_unchecked_mut(4 * i) as *mut IntMod<N2> as *mut __m256i;
                    ptr_add_eq_mul64(self_p2_ptr, a_p2_ptr, b_p2_ptr);
                }
            }
        } else {
            for i in 0..D {
                self.p1.points[i] += a.p1.points[i] * b.p1.points[i];
                self.p2.points[i] += a.p2.points[i] * b.p2.points[i];
            }
        }
    }
}

impl<
        'a,
        const D: usize,
        const N1: u64,
        const N2: u64,
        const N1_INV: u64,
        const N2_INV: u64,
        const W1: u64,
        const W2: u64,
    > AddAssign<&'a Self> for IntModCycloCRTEval<D, N1, N2, N1_INV, N2_INV, W1, W2>
{
    fn add_assign(&mut self, rhs: &'a Self) {
        self.p1 += &rhs.p1;
        self.p2 += &rhs.p2;
    }
}

impl<
        'a,
        const D: usize,
        const N1: u64,
        const N2: u64,
        const N1_INV: u64,
        const N2_INV: u64,
        const W1: u64,
        const W2: u64,
    > SubAssign<&'a Self> for IntModCycloCRTEval<D, N1, N2, N1_INV, N2_INV, W1, W2>
{
    fn sub_assign(&mut self, rhs: &'a Self) {
        self.p1 -= &rhs.p1;
        self.p2 -= &rhs.p2;
    }
}

// impl<'a, const D: usize, const N1: u64, const N2: u64, const N1_INV: u64, const N2_INV: u64, const W1: u64, const W2: u64> MulAssign<Z_N<N>> for Z_N_CycloNTT_CRT<D, N1, N2, N1_INV, N2_INV, W1, W2> {
//     fn mul_assign(&mut self, rhs: Z_N<N>) {
//         self.p1 *= &rhs.p1;
//         self.p2 *= &rhs.p2;
//     }
// }

impl<
        'a,
        const D: usize,
        const N1: u64,
        const N2: u64,
        const N1_INV: u64,
        const N2_INV: u64,
        const W1: u64,
        const W2: u64,
    > MulAssign<&'a Self> for IntModCycloCRTEval<D, N1, N2, N1_INV, N2_INV, W1, W2>
{
    fn mul_assign(&mut self, _: &'a Self) {
        todo!()
    }
}

impl<
        const D: usize,
        const N1: u64,
        const N2: u64,
        const N1_INV: u64,
        const N2_INV: u64,
        const W1: u64,
        const W2: u64,
        const BASE: u64,
        const LEN: usize,
    > RingElementDecomposable<BASE, LEN> for IntModCycloCRTEval<D, N1, N2, N1_INV, N2_INV, W1, W2>
{
    fn decompose_into_mat<const N: usize, const M: usize>(
        &self,
        mat: &mut Matrix<N, M, Self>,
        i: usize,
        j: usize,
    ) {
        let self_coeff = IntModCycloCRT::from(self);
        let mut decomps = Vec::<IntModDecomposition<BASE, LEN>>::with_capacity(D);
        for coeff_idx in 0..D {
            let coeff = u64::from(IntModCRT::<N1, N2, N1_INV, N2_INV>::from((
                self_coeff.p1.coeff[coeff_idx],
                self_coeff.p2.coeff[coeff_idx],
            )));
            decomps.push(IntModDecomposition::<BASE, LEN>::new(
                u64::from(coeff),
                N1 * N2,
            ));
        }
        for k in 0..LEN {
            let mut result_coeff = IntModCycloCRT::zero();
            for coeff_idx in 0..D {
                let result =
                    IntModCRT::<N1, N2, N1_INV, N2_INV>::from(decomps[coeff_idx].next().unwrap());
                result_coeff.p1.coeff[coeff_idx] = result.a1;
                result_coeff.p2.coeff[coeff_idx] = result.a2;
            }
            mat[(i + k, j)] = IntModCycloCRTEval::from(result_coeff);
        }
    }
}

/// Random sampling

impl<
        const D: usize,
        const N1: u64,
        const N2: u64,
        const N1_INV: u64,
        const N2_INV: u64,
        const W1: u64,
        const W2: u64,
    > RandUniformSampled for IntModCycloCRTEval<D, N1, N2, N1_INV, N2_INV, W1, W2>
{
    fn rand_uniform<T: Rng>(rng: &mut T) -> Self {
        IntModCycloCRTEval {
            p1: IntModCycloEval::rand_uniform(rng),
            p2: IntModCycloEval::rand_uniform(rng),
        }
    }
}

impl<
        const D: usize,
        const N1: u64,
        const N2: u64,
        const N1_INV: u64,
        const N2_INV: u64,
        const W1: u64,
        const W2: u64,
    > RandZeroOneSampled for IntModCycloCRTEval<D, N1, N2, N1_INV, N2_INV, W1, W2>
{
    fn rand_zero_one<T: Rng>(rng: &mut T) -> Self {
        (&IntModCycloCRT::rand_zero_one(rng)).into()
    }
}

impl<
        const D: usize,
        const N1: u64,
        const N2: u64,
        const N1_INV: u64,
        const N2_INV: u64,
        const W1: u64,
        const W2: u64,
    > RandDiscreteGaussianSampled for IntModCycloCRTEval<D, N1, N2, N1_INV, N2_INV, W1, W2>
{
    fn rand_discrete_gaussian<T: Rng, const NOISE_WIDTH_MILLIONTHS: u64>(rng: &mut T) -> Self {
        (&IntModCycloCRT::rand_discrete_gaussian::<_, NOISE_WIDTH_MILLIONTHS>(rng)).into()
    }
}

// Other polynomial-specific operations.

// TODO: maybe don't need this bc of index
// impl<const D: usize, const N1: u64, const N2: u64, const N1_INV: u64, const N2_INV: u64, const W1: u64, const W2: u64> Z_N_CycloNTT_CRT<D, N1, N2, N1_INV, N2_INV, W1, W2> {
//     pub fn coeff_iter(&self) -> Iter<'_, Z_N<{ N }>> {
//         self.coeff.iter()
//     }
// }

impl<
        const D: usize,
        const N1: u64,
        const N2: u64,
        const N1_INV: u64,
        const N2_INV: u64,
        const W1: u64,
        const W2: u64,
    > IntModCycloCRTEval<D, N1, N2, N1_INV, N2_INV, W1, W2>
{
    pub fn norm(&self) -> u64 {
        let p: IntModCycloCRT<D, N1, N2, N1_INV, N2_INV> = self.into();
        p.norm()
    }
}

impl<
        const D: usize,
        const N1: u64,
        const N2: u64,
        const N1_INV: u64,
        const N2_INV: u64,
        const W1: u64,
        const W2: u64,
    > IntModCycloCRTEval<D, N1, N2, N1_INV, N2_INV, W1, W2>
{
    pub fn reduce_mod(a: &mut IntModCycloCRTEval<D, 0, 0, 0, 0, 0, 0>) {
        for i in 0..D {
            a.p1.points[i] = IntMod::<N1>::from(u64::from(a.p1.points[i])).convert();
            a.p2.points[i] = IntMod::<N2>::from(u64::from(a.p2.points[i])).convert();
        }
    }

    pub fn auto(&self, k: usize) -> Self {
        (self.p1.auto(k), self.p2.auto(k)).into()
    }

    pub fn mul_x_pow(&self, k: usize) -> Self {
        (self.p1.mul_x_pow(k), self.p2.mul_x_pow(k)).into()
    }
}

unsafe impl<
        const D: usize,
        const N1: u64,
        const N2: u64,
        const N1_INV: u64,
        const N2_INV: u64,
        const W1: u64,
        const W2: u64,
        const M1: u64,
        const M2: u64,
        const M1_INV: u64,
        const M2_INV: u64,
        const WW1: u64,
        const WW2: u64,
    > RingCompatible<IntModCycloCRTEval<D, M1, M2, M1_INV, M2_INV, WW1, WW2>>
    for IntModCycloCRTEval<D, N1, N2, N1_INV, N2_INV, W1, W2>
{
}

// TODO: this should be a TryFrom
impl<
        const D: usize,
        const N1: u64,
        const N2: u64,
        const N1_INV: u64,
        const N2_INV: u64,
        const W1: u64,
        const W2: u64,
    > From<&IntModCycloCRTEval<D, N1, N2, N1_INV, N2_INV, W1, W2>>
    for IntModCRT<N1, N2, N1_INV, N2_INV>
{
    fn from(a: &IntModCycloCRTEval<D, N1, N2, N1_INV, N2_INV, W1, W2>) -> Self {
        let p: IntModCycloCRT<D, N1, N2, N1_INV, N2_INV> = a.into();
        (&p).into()
    }
}

// impl<const D: usize, const N1: u64, const N2: u64, const N1_INV: u64, const N2_INV: u64, const W1: u64, const W2: u64> Index<usize> for Z_N_CycloNTT_CRT<D, N1, N2, N1_INV, N2_INV, W1, W2> {
//     type Output = (Z_N<N1>, Z_N<N2>);
//     fn index(&self, index: usize) -> &Self::Output {
//         &(self.p1[0], self.p2[0])
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::math::matrix::Matrix;

//     const D: usize = 4; // Z_q[X] / (X^4 + 1)
//     const P: u64 = (1_u64 << 32) - 5;

//     // TODO: add more tests.
//     #[test]
//     fn test_from_into() {
//         let p = Z_N_CycloNTT_CRT::<D, P>::from(vec![42, 6, 1, 0, 5]);
//         let q = Z_N_CycloNTT_CRT::<D, P>::from(vec![37, 6, 1, 0]);
//         let r = Z_N_CycloNTT_CRT::<D, P>::from(vec![41, 6, 1, 0, 5, 0, 0, 0, 1]);
//         assert_eq!(p, q);
//         assert_eq!(p, r);
//         assert_eq!(q, r);

//         let s = Z_N_CycloNTT_CRT::<D, P>::from(vec![9483, 1, 1, 1, 323, P - 12139, 10491, 1, 1]);
//         let t = Z_N_CycloNTT_CRT::<D, P>::from(vec![9161, 12140, P - 10490, 0, 0]);
//         assert_eq!(s, t);
//     }

//     #[test]
//     fn test_ops() {
//         let p = Z_N_CycloNTT_CRT::<D, P>::from(vec![0, 0, 0, 1]);
//         let q = Z_N_CycloNTT_CRT::<D, P>::from(vec![0, 0, 2, 0]);
//         let sum = Z_N_CycloNTT_CRT::<D, P>::from(vec![0, 0, 2, 1]);
//         let diff = Z_N_CycloNTT_CRT::<D, P>::from(vec![0, 0, P - 2, 1]);
//         let prod = Z_N_CycloNTT_CRT::<D, P>::from(vec![0, P - 2, 0, 0]);
//         let square = Z_N_CycloNTT_CRT::<D, P>::from(vec![0, 0, P - 1, 0]);
//         let neg = Z_N_CycloNTT_CRT::<D, P>::from(vec![0, 0, 0, P - 1]);
//         assert_eq!(&p + &q, sum);
//         assert_eq!(&p - &q, diff);
//         assert_eq!(&p * &q, prod);
//         assert_eq!(&p * &p, square);
//         assert_eq!(-&p, neg);
//     }

//     #[test]
//     fn test_matrix() {
//         let mut M: Matrix<2, 2, Z_N_CycloNTT_CRT<D, P>> = Matrix::zero();
//         M[(0, 0)] = Z_N_CycloNTT_CRT::<D, P>::from(vec![0, 0, 0, 1]);
//         M[(0, 1)] = Z_N_CycloNTT_CRT::<D, P>::from(vec![0, 0, 1, 0]);
//         M[(1, 0)] = Z_N_CycloNTT_CRT::<D, P>::from(vec![0, 1, 0, 0]);
//         M[(1, 1)] = Z_N_CycloNTT_CRT::<D, P>::from(vec![1, 0, 0, 0]);
//         // M =
//         // [ x^3 x^2 ]
//         // [ x   1   ]
//         let M_square = &M * &M;
//         assert_eq!(
//             M_square[(0, 0)],
//             Z_N_CycloNTT_CRT::<D, P>::from(vec![0, 0, P - 1, 1])
//         ); // x^3 + x^6
//         assert_eq!(
//             M_square[(0, 1)],
//             Z_N_CycloNTT_CRT::<D, P>::from(vec![0, P - 1, 1, 0])
//         ); // x^2 + x^5
//         assert_eq!(
//             M_square[(1, 0)],
//             Z_N_CycloNTT_CRT::<D, P>::from(vec![P - 1, 1, 0, 0])
//         ); // x + x^4
//         assert_eq!(
//             M_square[(1, 1)],
//             Z_N_CycloNTT_CRT::<D, P>::from(vec![1, 0, 0, 1])
//         ); // 1 + x^3

//         let M_double = &M + &M;
//         assert_eq!(
//             M_double[(0, 0)],
//             Z_N_CycloNTT_CRT::<D, P>::from(vec![0, 0, 0, 2])
//         );
//         assert_eq!(
//             M_double[(0, 1)],
//             Z_N_CycloNTT_CRT::<D, P>::from(vec![0, 0, 2, 0])
//         );
//         assert_eq!(
//             M_double[(1, 0)],
//             Z_N_CycloNTT_CRT::<D, P>::from(vec![0, 2, 0, 0])
//         );
//         assert_eq!(
//             M_double[(1, 1)],
//             Z_N_CycloNTT_CRT::<D, P>::from(vec![2, 0, 0, 0])
//         );

//         let M_neg = -&M;
//         assert_eq!(
//             M_neg[(0, 0)],
//             Z_N_CycloNTT_CRT::<D, P>::from(vec![0, 0, 0, P - 1])
//         );
//         assert_eq!(
//             M_neg[(0, 1)],
//             Z_N_CycloNTT_CRT::<D, P>::from(vec![0, 0, P - 1, 0])
//         );
//         assert_eq!(
//             M_neg[(1, 0)],
//             Z_N_CycloNTT_CRT::<D, P>::from(vec![0, P - 1, 0, 0])
//         );
//         assert_eq!(
//             M_neg[(1, 1)],
//             Z_N_CycloNTT_CRT::<D, P>::from(vec![P - 1, 0, 0, 0])
//         );
//     }
// }
