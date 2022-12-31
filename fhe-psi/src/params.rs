/*
 *  The goal is *eventually* once Rust implements everything
 *  we can pass around Param consts as type parameters, so that
 *  keys / ciphertexts / etc. are bound by type to their parameters,
 *  and matrix dims, moduli, etc. can all be inferred from type.
 *
 *  For now, we have massive jank.
 */

/*
 * Parameters:
 *   - N,M: matrix dimensions.
 *   - P: plaintext modulus.
 *   - Q: ciphertext modulus.
 *   - G_BASE: base used for the gadget matrix.
 *   - G_LEN: length of the "g" gadget vector, or alternatively log q.
 *   - N_MINUS_1: N-1, since generics cannot be used in const expressions yet. Used only in key generation.
 *   - NG_LEN: N * G_LEN, since generics cannot be used in const expressions yet. Used only in ciphertext multiplication.
 */

pub struct IntParams<const N: usize, const M: usize, const P: u64, const Q: u64, const G_BASE: u64, const G_LEN: usize, const N_MINUS_1: usize> {
    pub noise_width: f64,
}

pub fn verify_int_params<const N: usize, const M: usize, const P: u64, const Q: u64, const G_BASE: u64, const G_LEN: usize, const N_MINUS_1: usize>(params: IntParams<N,M,P,Q,G_BASE,G_LEN,N_MINUS_1>) {
    assert!(N_MINUS_1+1 == N);
    assert!(P <= Q);

    // G_LEN >= log Q
    let mut x = Q;
    for _ in 0..G_LEN {
        x /= G_BASE;
    }
    assert!(x == 0);

    // M >= N * G_LEN >= N log Q
    assert!(G_LEN * N <= M);

}

pub const DumbParams : IntParams<5, 140, 10, 268369921, 2, 28, 4> = IntParams {
    noise_width: 6.4,
};


// pub struct Params {
//     pub poly_len: usize,
//     pub poly_len_log2: usize,
//     pub ntt_tables: Vec<Vec<Vec<u64>>>,
//     pub scratch: Vec<u64>,

//     pub crt_count: usize,
//     pub barrett_cr_0: [u64; MAX_MODULI],
//     pub barrett_cr_1: [u64; MAX_MODULI],
//     pub barrett_cr_0_modulus: u64,
//     pub barrett_cr_1_modulus: u64,
//     pub mod0_inv_mod1: u64,
//     pub mod1_inv_mod0: u64,
//     pub moduli: [u64; MAX_MODULI],
//     pub modulus: u64,
//     pub modulus_log2: u64,
//     pub noise_width: f64,

//     pub n: usize,
//     pub pt_modulus: u64,
//     pub q2_bits: u64,
//     pub t_conv: usize,
//     pub t_exp_left: usize,
//     pub t_exp_right: usize,
//     pub t_gsw: usize,

//     pub expand_queries: bool,
//     pub db_dim_1: usize,
//     pub db_dim_2: usize,
//     pub instances: usize,
//     pub db_item_size: usize,
// }

