#![no_std]

use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct HappyContract;

#[contractimpl]
impl HappyContract {
    pub fn add(env: Env, a: i32, b: i32) -> i32 {
        let _ = env.storage().persistent().get::<_, i32>(&0i32);
        a + b
    }

    pub fn noop(_env: Env) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use soroban_cost_harness::record;

    use super::*;

    #[test]
    fn measure_add() {
        let env = Env::default();
        let contract_id = env.register(HappyContract, ());
        let client = HappyContractClient::new(&env, &contract_id);

        let result = record(&env, "add", || client.add(&3, &4));
        assert_eq!(result, 7);
    }

    #[test]
    fn measure_noop() {
        let env = Env::default();
        let contract_id = env.register(HappyContract, ());
        let client = HappyContractClient::new(&env, &contract_id);

        let result = record(&env, "noop", || client.noop());
        assert_eq!(result, 0);
    }
}
