use api3_contract::NearWhitelistContract;
pub use near_sdk::json_types::{Base64VecU8, ValidAccountId, WrappedDuration, U64};
use near_sdk::serde_json::json;
use near_sdk_sim::{call, deploy, init_simulator, view, ContractAccount, UserAccount};

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    CONTRACT_BYTES => "../client-test/out/main.wasm",
}

pub const DEFAULT_GAS: u64 = 300_000_000_000_000;

fn init() -> (UserAccount, ContractAccount<NearWhitelistContract>) {
    let root = init_simulator(None);

    // Deploy the compiled Wasm bytes
    let counter: ContractAccount<NearWhitelistContract> = deploy!(
        contract: NearWhitelistContract,
        contract_id: "near_counter".to_string(),
        bytes: &CONTRACT_BYTES,
        signer_account: root
    );

    (root, counter)
}

#[test]
fn simulate_increment() {
    let (root, counter) = init();

    let contract_info: String = view!(counter.contract_info()).unwrap_json();
    println!("{}", &contract_info);
}
