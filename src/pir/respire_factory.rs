use std::time::Instant;
use crate::pir::pir::{PIRRecordBytes, TimeStats, PIR};
use crate::pir::respire::{RespireParams, RespireParamsExpanded};
use crate::respire;
use clap::Parser;
use itertools::Itertools;
use log::info;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

//
// Parameter factory functions
//

#[allow(non_snake_case)]
pub struct FactoryParams {
    pub BATCH_SIZE: usize,
    pub N_VEC: usize,
    pub P: u64,
    pub D_RECORD: usize,
    pub NU1: usize,
    pub NU2: usize,
    pub Q_SWITCH1: u64,
    pub Q_SWITCH2: u64,
    pub D_SWITCH: usize,
    pub WIDTH_SWITCH_MILLIONTHS: u64,
}

impl FactoryParams {
    pub const fn expand(&self) -> RespireParams {
        RespireParams {
            Q_A: 268369921,
            Q_B: 249561089,
            D: 2048,
            T_GSW: 8,
            T_REGEV_TO_GSW: 4,
            T_AUTO_REGEV: 3,
            T_AUTO_GSW: 9,
            T_SCAL_TO_VEC: 8,
            BATCH_SIZE: self.BATCH_SIZE,
            N_VEC: self.N_VEC,
            ERROR_WIDTH_MILLIONTHS: 9_900_000,
            ERROR_WIDTH_VEC_MILLIONTHS: 9_900_000,
            ERROR_WIDTH_SWITCH_MILLIONTHS: self.WIDTH_SWITCH_MILLIONTHS,
            SECRET_BOUND: 7,
            SECRET_WIDTH_VEC_MILLIONTHS: 9_900_000,
            SECRET_WIDTH_SWITCH_MILLIONTHS: self.WIDTH_SWITCH_MILLIONTHS,
            P: self.P,
            D_RECORD: self.D_RECORD,
            NU1: self.NU1,
            NU2: self.NU2,
            Q_SWITCH1: self.Q_SWITCH1,
            Q_SWITCH2: self.Q_SWITCH2,
            D_SWITCH: self.D_SWITCH,
        }
    }
}

// For quick testing

pub const RESPIRE_TEST_PARAMS: RespireParamsExpanded = FactoryParams {
    BATCH_SIZE: 1,
    N_VEC: 1,
    P: 16,
    D_RECORD: 512,
    NU1: 9,
    NU2: 9,
    Q_SWITCH1: 8 * 16,
    Q_SWITCH2: 1032193, // 19.97 bits
    D_SWITCH: 512,
    WIDTH_SWITCH_MILLIONTHS: 46_000_000,
}
.expand()
.expand();

pub type RespireTest = respire!(RESPIRE_TEST_PARAMS);

#[cfg(not(target_feature = "avx2"))]
pub fn has_avx2() -> bool {
    false
}

#[cfg(target_feature = "avx2")]
pub fn has_avx2() -> bool {
    true
}

// TODO encapsulate stats into struct instead of printing directly
// struct RunResult {
//     success: bool,
//     noise: f64,
//     // preprocess_time: Duration,
//     // setup_time: Duration,
//     query_time: Duration,
//     answer_time: Duration,
//     extract_time: Duration,
// }

pub fn run_pir<ThePIR: PIR, I: Iterator<Item = usize>>(iter: I) {
    eprintln!("Running PIR...");
    eprintln!(
        "AVX2 is {}",
        if has_avx2() {
            "enabled"
        } else {
            "not enabled "
        }
    );
    eprintln!("========");
    ThePIR::print_summary();
    eprintln!("========");

    let records_generator = |i: usize| {
        let mut record = vec![0_u8; ThePIR::BYTES_PER_RECORD];
        record[0] = (i % 256) as u8;
        record[1] = ((i / 256) % 256) as u8;
        record[2] = 42_u8;
        record[3] = 0_u8;
        record[4] = (i % 100) as u8;
        record[5] = ((i / 100) % 100) as u8;
        record[6] = ((i / 100 / 100) % 100) as u8;
        record[7] = ((i / 100 / 100 / 100) % 100) as u8;
        // for i in 8..256 {
        //     record[i] = random();
        // }
        ThePIR::RecordBytes::from_bytes(record.as_slice()).unwrap()
    };

    let mut init_times = TimeStats::new();
    let (db, db_hint) = ThePIR::encode_db(records_generator, Some(&mut init_times));
    let (qk, pp) = ThePIR::setup(Some(&mut init_times));
    info!("Init times:\n{}", init_times);
    info!("Total time = {:?}", init_times.total());

    let run_trial = |indices: &[usize]| {
        eprintln!("Running trial on indices {:?}", &indices);
        assert_eq!(indices.len(), ThePIR::BATCH_SIZE);
        let mut trial_times = TimeStats::new();

        let begin = Instant::now();
        let (q, st) = ThePIR::query(&qk, indices, &db_hint, Some(&mut trial_times));
        let response = ThePIR::answer(&pp, &db, &q, Some(&qk), Some(&mut trial_times));
        let extracted = ThePIR::extract(&qk, &response, &st, Some(&mut trial_times));
        let end = Instant::now();

        info!("Trial times:\n{}\n** total: {:?}", trial_times,  trial_times.total());
        eprintln!("End to end time: {:?}", end - begin);

        for (idx, decoded_record) in indices.iter().copied().zip(extracted) {
            if decoded_record.as_bytes() != records_generator(idx).as_bytes() {
                eprintln!("**** **** **** **** ERROR **** **** **** ****");
                eprintln!("protocol failed");
                eprintln!("idx = {}", idx);
                eprintln!("decoded record = {:?}", decoded_record.as_bytes());
                eprintln!("actual record = {:?}", records_generator(idx).as_bytes());
            }
        }
    };

    for chunk in iter.chunks(ThePIR::BATCH_SIZE).into_iter() {
        let c_vec = chunk.collect_vec();
        run_trial(c_vec.as_slice());
    }
}

#[macro_export]
macro_rules! generate_main {
    ($name: path) => {
        fn main() {
            $crate::pir::respire_factory::factory_main::<$name>();
        }
    };
}

#[derive(Parser, Debug)]
struct Args {
    trials: usize,
}

pub fn factory_main<ThePIR: PIR>() {
    env_logger::init();
    let args = Args::parse();

    let mut rng = ChaCha20Rng::from_entropy();
    let record_gen = |_| rng.gen_range(0_usize..ThePIR::NUM_RECORDS);
    run_pir::<ThePIR, _>((0usize..args.trials * ThePIR::BATCH_SIZE).map(record_gen));
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::math::int_mod::IntMod;
    use crate::math::int_mod_cyclo::IntModCyclo;
    use crate::math::int_mod_poly::IntModPoly;
    use crate::math::matrix::Matrix;
    use crate::pir::respire::Respire;

    #[test]
    fn test_regev() {
        let s = RespireTest::encode_setup();
        let mu = <RespireTest as Respire>::RingP::from(12_u64);
        let encoded = RespireTest::encode_regev(&s, &mu.scale_up_into());
        let decoded: <RespireTest as Respire>::RingP =
            RespireTest::decode_regev(&s, &encoded).round_down_into();
        assert_eq!(mu, decoded);
    }

    #[test]
    fn test_gsw() {
        let s = RespireTest::encode_setup();
        type RingPP = IntModCyclo<{ RESPIRE_TEST_PARAMS.D }, 1024>;
        let mu = RingPP::from(111_u64);
        let encrypt = RespireTest::encode_gsw(&s, &mu.include_into());

        let scale = <RespireTest as Respire>::RingQFast::from(RESPIRE_TEST_PARAMS.Q / 1024);
        let decrypt = RespireTest::decode_gsw_scaled(&s, &encrypt, &scale);
        assert_eq!(decrypt.round_down_into(), mu);
    }

    #[test]
    fn test_auto_hom() {
        let s = RespireTest::encode_setup();
        let auto_key = RespireTest::auto_setup::<
            { RESPIRE_TEST_PARAMS.T_AUTO_REGEV },
            { RESPIRE_TEST_PARAMS.Z_AUTO_REGEV },
        >(3, &s);
        let x = <RespireTest as Respire>::RingP::from(IntModPoly::x());
        let encrypt = RespireTest::encode_regev(&s, &x.scale_up_into());
        let encrypt_auto = RespireTest::auto_hom::<
            { RESPIRE_TEST_PARAMS.T_AUTO_REGEV },
            { RESPIRE_TEST_PARAMS.Z_AUTO_REGEV },
        >(&auto_key, &encrypt);
        let decrypt: <RespireTest as Respire>::RingP =
            RespireTest::decode_regev(&s, &encrypt_auto).round_down_into();
        assert_eq!(decrypt, &(&x * &x) * &x);
    }

    #[test]
    fn test_regev_to_gsw() {
        let s = RespireTest::encode_setup();
        let s_regev_to_gsw = RespireTest::regev_to_gsw_setup(&s);
        type RingPP = IntModCyclo<{ RESPIRE_TEST_PARAMS.D }, 1024>;
        let mu = RingPP::from(567_u64);
        let mut mu_curr = mu.include_into();
        let mut encrypt_vec = Vec::with_capacity(RESPIRE_TEST_PARAMS.T_GSW);
        for _ in 0..RESPIRE_TEST_PARAMS.T_GSW {
            encrypt_vec.push(RespireTest::encode_regev(&s, &mu_curr));
            mu_curr *= IntMod::from(RESPIRE_TEST_PARAMS.Z_GSW);
        }
        let encrypt_gsw = RespireTest::regev_to_gsw(&s_regev_to_gsw, encrypt_vec.as_slice());

        let scale = <RespireTest as Respire>::RingQFast::from(RESPIRE_TEST_PARAMS.Q / 1024);
        let decrypted = RespireTest::decode_gsw_scaled(&s, &encrypt_gsw, &scale);
        assert_eq!(decrypted.round_down_into(), mu);
    }

    #[test]
    fn test_scal_to_vec() {
        let s_scal = RespireTest::encode_setup();
        let s_vec = RespireTest::encode_vec_setup();
        let s_scal_to_vec = RespireTest::scal_to_vec_setup(&s_scal, &s_vec);

        let mut cs = Vec::<<RespireTest as Respire>::RegevCiphertext>::with_capacity(
            RESPIRE_TEST_PARAMS.N_VEC,
        );
        let mut expected =
            Matrix::<{ RESPIRE_TEST_PARAMS.N_VEC }, 1, <RespireTest as Respire>::RingP>::zero();
        for i in 0..RESPIRE_TEST_PARAMS.N_VEC {
            let mu = <RespireTest as Respire>::RingP::from(i as u64 + 1_u64);
            expected[(i, 0)] = mu.clone();
            cs.push(RespireTest::encode_regev(&s_scal, &mu.scale_up_into()));
        }

        let c_vec = RespireTest::scal_to_vec(&s_scal_to_vec, cs.as_slice().try_into().unwrap());
        let decoded = RespireTest::decode_vec_regev(&s_vec, &c_vec);
        let actual = decoded.map_ring(|r| r.round_down_into());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_post_process_only() {
        let (qk, pp) = RespireTest::setup(None);
        let (_, s_vec, _) = &qk;
        let mut m = <RespireTest as Respire>::RecordPackedSmall::zero();
        for i in 0..RESPIRE_TEST_PARAMS.N_VEC {
            m[(i, 0)] = IntModCyclo::from(177_u64 + i as u64)
        }
        let c =
            RespireTest::encode_vec_regev(s_vec, &m.map_ring(|r| r.include_dim().scale_up_into()));
        let compressed = RespireTest::answer_compress_vec(&pp, &c, RESPIRE_TEST_PARAMS.N_VEC);
        let extracted = RespireTest::extract_ring_one(&qk, &compressed);
        assert_eq!(m, extracted);
    }

    // #[test]
    // fn test_encode_decode() {
    //     // 16 x (log2(9) = 3 bits) <=> 6 bytes
    //     let bytes = [48_u8, 47, 17, 255, 183, 0];
    //     // 00110000 00101111 00010001 11111111 10110111 00000000
    //     // 001 100 000 010 111 100 010 001 111 111 111 011 011 100 000 000
    //     let encoded = RespireTest::encode_record(&bytes);
    //     assert_eq!(
    //         encoded,
    //         IntModCyclo::from(
    //             [1_u64, 4, 0, 2, 7, 4, 2, 1, 7, 7, 7, 3, 3, 4, 0, 0].map(IntMod::from)
    //         )
    //     );
    //     let decoded = RespireTest::decode_record(&encoded);
    //     assert_eq!(bytes, decoded);
    // }

    #[test]
    fn test_respire_one() {
        run_pir::<RespireTest, _>([711_711].into_iter());
    }
}