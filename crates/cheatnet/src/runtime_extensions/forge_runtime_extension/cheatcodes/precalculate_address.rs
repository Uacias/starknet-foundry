use crate::CheatnetState;
use crate::runtime_extensions::common::create_execute_calldata;
use conversions::IntoConv;
use runtime::starknet::constants::TEST_ADDRESS;
use starknet_api::core::{ClassHash, ContractAddress, calculate_contract_address};
use starknet_types_core::felt::Felt;

impl CheatnetState {
    #[must_use]
    pub fn precalculate_address(
        &self,
        class_hash: &ClassHash,
        calldata: &[Felt],
    ) -> ContractAddress {
        let salt = self.get_salt();

        let execute_calldata = create_execute_calldata(calldata);
        let deployer_address = Felt::from_hex(TEST_ADDRESS).unwrap();
        calculate_contract_address(
            salt,
            *class_hash,
            &execute_calldata,
            deployer_address.into_(),
        )
        .unwrap()
    }
}

pub fn precalculate_address_with(
    salt: starknet_api::transaction::fields::ContractAddressSalt,
    deployer_address: ContractAddress,
    class_hash: &ClassHash,
    calldata: &[Felt],
) -> ContractAddress {
    let execute_calldata = create_execute_calldata(calldata);

    calculate_contract_address(salt, *class_hash, &execute_calldata, deployer_address)
        .expect("failed to compute address")
}

#[cfg(test)]
mod get_addr_tests {
    use conversions::padded_felt::PaddedFelt;
    use starknet::core::utils::get_contract_address;
    use starknet_types_core::felt::Felt;
    // USE THIS TO PRECOMPUTE THE PRIVACY ADDRESS
    #[test]
    fn new_constructor_args_yields_expected_address() {
        // 1) salt - use the same with sncast
        let salt = Felt::from_hex_unchecked("0x123");
        // 2) class hash of privacy
        let class_hash = Felt::from_hex_unchecked(
            "0x02287b34a375448b9a263f49ac4b46c4da4007f774c5c529ed4cf3a0c5bfff56",
        );
        // 3) deployer - it's fine, leave it as 0
        let deployer = Felt::ZERO;
        // 4) calldata
        let constructor_calldata = &[
            Felt::from_hex_unchecked(
                "0x0796810235fC228FbBC232b3070135823490e9e3d61781969d7C5eD890d2c468", // admin
            ),
            Felt::from_hex_unchecked(
                "0x0796810235fC228FbBC232b3070135823490e9e3d61781969d7C5eD890d2c468", // upgrader
            ),
            Felt::from_hex_unchecked(
                "0x076d5ef47375ddcfff2e2cf2f3c0bd951d6aa3be85562951d729888e3f36a7d9", // recursive verifier
            ),
            Felt::from_hex_unchecked(
                "0x4718f5a0fc34cc1af16a1cdee98ffb20c31f5cd61d6ab07201858f4287c938d", // strk address
            ),
            Felt::from_hex_unchecked("0x4563918244F40000"), // 5 STRK initial buffer
            Felt::from_hex_unchecked("0x0"),
            Felt::from_hex_unchecked("0x3e30fb297e5d"), // gas price get this from https://www.alchemy.com/docs/node/starknet/starknet-api-endpoints/starknet-get-block-with-tx-hashes?explorer=true
            Felt::from_hex_unchecked("0x0"),
            Felt::from_hex_unchecked("0x61a8"), // gas consumed for merge
            Felt::from_hex_unchecked("0x6590"), // gas consumed for transfer
        ];

        println!("Calldata {:?}", constructor_calldata);

        let addr = get_contract_address(salt, class_hash, constructor_calldata, deployer);
        let a2 = PaddedFelt(addr);
        println!("addr {:?}", addr);
        println!("a2 {:?}", a2.0);
    }
}
