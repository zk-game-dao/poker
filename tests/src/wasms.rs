use std::{fs, io::Read, path::Path};

use lazy_static::lazy_static;

pub type CanisterWasm = Vec<u8>;

lazy_static! {
    pub static ref TABLE: CanisterWasm = get_local_wasm("table_canister");
    pub static ref TABLE_INDEX: CanisterWasm = get_local_wasm("table_index");
    pub static ref TOURNAMENT: CanisterWasm = get_local_wasm("tournament_canister");
    pub static ref TOURNAMENT_INDEX: CanisterWasm = get_local_wasm("tournament_index");
    pub static ref USER: CanisterWasm = get_local_wasm("users_canister");
    pub static ref USER_INDEX: CanisterWasm = get_local_wasm("users_index");
    pub static ref CYCLE_DISPENSER_CANISTER: CanisterWasm = get_local_wasm("cycle_dispenser");
    pub static ref LEDGER: CanisterWasm = get_remote_wasm("icp_ledger");
    pub static ref ICRC1_LEDGER: CanisterWasm = get_remote_wasm("ic-icrc1-ledger");
}

fn get_canister_wasm<P: AsRef<Path>>(path: P) -> CanisterWasm {
    let path = path.as_ref();

    let mut file = fs::File::open(path).unwrap_or_else(|e| {
        panic!("Failed to open file: {}, reason: {}", path.display(), e);
    });
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap_or_else(|e| {
        panic!("Failed to read file: {}, reason: {}", path.display(), e);
    });
    bytes
}

fn get_local_wasm(canister_name: &str) -> CanisterWasm {
    let path = format!(
        "../target/wasm32-unknown-unknown/release/{}.wasm",
        canister_name
    );

    get_canister_wasm(path)
}

fn get_remote_wasm(canister_name: &str) -> CanisterWasm {
    let path = format!("wasms/{}.wasm.gz", canister_name);

    get_canister_wasm(path)
}
