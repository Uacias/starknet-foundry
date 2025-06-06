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
        let salt = Felt::from_hex_unchecked("0x666");
        // 2) class hash of privacy
        let class_hash = Felt::from_hex_unchecked(
            "0x069a30e4c1deff60d7c19184ecccf8f1db30295aaed4d7ea5652d40075b3ff89",
        );
        // 3) deployer - it's fine, leave it as 0
        let deployer = Felt::ZERO;
        // 4) calldata
        let constructor_calldata = &[
            Felt::from_hex_unchecked(
                "0x064fa47c02430E5d69c0c5d340e23397bca308f7B9d85247565fc91F2C2aD2f2", // owner
            ),
            Felt::from_hex_unchecked(
                "0x045593366c8421de54531d304da34f4e85dbcd30191d6941973dbd7f3a71c4da", // execute verifier
            ),
            Felt::from_hex_unchecked(
                "0x4718f5a0fc34cc1af16a1cdee98ffb20c31f5cd61d6ab07201858f4287c938d", // strk address
            ),
            Felt::from_hex_unchecked("0x2b5e3af16b1880000"), // 50 STRK initial buffer
            Felt::from_hex_unchecked("0x0"),
            Felt::from_hex_unchecked("0x22d68309d6cc"), // gas price get this from https://www.alchemy.com/docs/node/starknet/starknet-api-endpoints/starknet-get-block-with-tx-hashes?explorer=true
            Felt::from_hex_unchecked("0x0"),
            Felt::from_hex_unchecked("0x61a8"), // gas consumed for merge
            Felt::from_hex_unchecked("0x6590"), // gas consumed for transfer
        ];

        let addr = get_contract_address(salt, class_hash, constructor_calldata, deployer);
        let a2 = PaddedFelt(addr);
        println!("addr {:?}", addr);
        println!("a2 {:?}", a2.0);
    }
}
