#![no_std]

use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct DupContract;

#[contractimpl]
impl DupContract {
    pub fn add(env: Env, a: i32, b: i32) -> i32 {
        let _ = env.storage().persistent().get::<_, i32>(&0i32);
        a + b
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use soroban_cost_harness::record;

    use super::*;

    #[test]
    fn measure_add_small() {
        let env = Env::default();
        let contract_id = env.register(DupContract, ());
        let client = DupContractClient::new(&env, &contract_id);

        let result = record(&env, "add", || client.add(&1, &2));
        assert_eq!(result, 3);
    }

    #[test]
    fn measure_add_large() {
        let env = Env::default();
        let contract_id = env.register(DupContract, ());
        let client = DupContractClient::new(&env, &contract_id);

        let result = record(&env, "add", || client.add(&100, &200));
        assert_eq!(result, 300);
    }
}
