use respire::pir::cuckoo_respire::CuckooRespireImpl;
use respire::pir::respire::RespireParamsExpanded;
use respire::pir::respire_harness::FactoryParams;
use respire::{generate_main, respire};

// TODO: Check params
const BASE_PARAMS: RespireParamsExpanded = FactoryParams::batch_256(61, 8, 9, 8).expand().expand();

type BasePIR = respire!(BASE_PARAMS);
type CuckooPIR = CuckooRespireImpl<40, 61, { 2usize.pow(24) }, BasePIR>;

generate_main!(CuckooPIR);
