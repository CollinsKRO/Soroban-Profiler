#![no_std]

use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct CompileErrorContract;

#[contractimpl]
impl CompileErrorContract {
    // This function has a deliberate syntax error: missing semicolon.
    // cargo test should fail to compile, and the profiler should surface
    // the compiler error rather than silently producing a partial report.
    pub fn broken(env: Env) -> i32 {
        let x = 42
        x
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use soroban_cost_harness::record;

    use super::*;

    #[test]
    fn measure_broken() {
        let env = Env::default();
        let contract_id = env.register(CompileErrorContract, ());
        let client = CompileErrorContractClient::new(&env, &contract_id);

        record(&env, "broken", || client.broken());
    }
}
