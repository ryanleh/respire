use respire::pir::cuckoo_respire::CuckooRespireImpl;
use respire::pir::respire::RespireParamsExpanded;
use respire::pir::respire_harness::FactoryParams;
use respire::{generate_main, respire};

// TODO: Check params, check n_vec=7
const BASE_PARAMS: RespireParamsExpanded = FactoryParams::batch_256(37, 8, 9, 8).expand().expand();

type BasePIR = respire!(BASE_PARAMS);
type CuckooPIR = CuckooRespireImpl<24, 37, { 2usize.pow(22) }, BasePIR>;

generate_main!(CuckooPIR);