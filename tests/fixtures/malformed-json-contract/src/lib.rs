#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct MalformedJsonContract;

#[contractimpl]
impl MalformedJsonContract {
    pub fn noop(_env: Env) {}
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;
    use soroban_cost_harness::record;

    #[test]
    fn valid_record() {
        let env = Env::default();
        let contract_id = env.register(MalformedJsonContract, ());
        let client = MalformedJsonContractClient::new(&env, &contract_id);

        record(&env, "valid_fn", || {
            client.noop();
        });
    }

    #[test]
    fn malformed_record() {
        // Print a deliberately truncated marker line (cut off mid-JSON)
        std::println!("##SOROBAN_COST_JSON##{{\"label\":\"bad_fn\",\"cpu_instructions\":1000");
        // Also print a valid record from the same test to ensure both are handled
        let env = Env::default();
        let contract_id = env.register(MalformedJsonContract, ());
        let client = MalformedJsonContractClient::new(&env, &contract_id);

        record(&env, "also_valid", || {
            client.noop();
        });
    }
}
