use respire::pir::cuckoo_respire::CuckooRespireImpl;
use respire::pir::respire::RespireParamsExpanded;
use respire::pir::respire_harness::FactoryParams;
use respire::{generate_main, respire};

// TODO: Check params
const BASE_PARAMS: RespireParamsExpanded = FactoryParams::batch_256(73, 8, 8, 8).expand().expand();

type BasePIR = respire!(BASE_PARAMS);
type CuckooPIR = CuckooRespireImpl<48, 73, { 2usize.pow(24) }, BasePIR>;

generate_main!(CuckooPIR);
