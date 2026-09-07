#![no_std]

use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct PanicContract;

#[contractimpl]
impl PanicContract {
    pub fn double(env: Env, x: i32) -> i32 {
        let _ = env.storage().persistent().get::<_, i32>(&0i32);
        x * 2
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use soroban_cost_harness::record;

    use super::*;

    #[test]
    fn measure_double() {
        let env = Env::default();
        let contract_id = env.register(PanicContract, ());
        let client = PanicContractClient::new(&env, &contract_id);

        let result = record(&env, "double", || client.double(&5));
        assert_eq!(result, 10);
    }

    #[test]
    fn measure_double_panic() {
        let env = Env::default();
        let contract_id = env.register(PanicContract, ());
        let client = PanicContractClient::new(&env, &contract_id);

        // This record call will emit the JSON line, then the closure panics.
        record(&env, "double_panic", || {
            let val = client.double(&3);
            // Panic mid-execution after the record call has captured resources.
            assert_eq!(val, 6, "expected 6 but got {val}");
            panic!("intentional panic for testing profiler error handling");
        });
    }
}
