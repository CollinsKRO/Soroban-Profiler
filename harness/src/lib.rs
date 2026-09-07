use serde::Serialize;
use soroban_sdk::Env;

/// Machine-readable cost record emitted to stdout during `cargo test --nocapture`.
/// The CLI crate greps for the `##SOROBAN_COST_JSON##` prefix to extract these.
#[derive(Serialize)]
pub struct CostRecord {
    pub label: String,
    pub cpu_instructions: u64,
    pub memory_bytes: u64,
}

/// Measure the Soroban resource cost of running `f` and emit a JSON line to stdout.
///
/// The budget is reset to unlimited before each call so every `record()`
/// invocation measures only the cost of `f` in isolation.
///
/// # Example
///
/// ```ignore
/// use soroban_sdk::{contract, contractimpl, Env};
/// use soroban_cost_harness::record;
///
/// #[contract]
/// pub struct Counter;
///
/// #[contractimpl]
/// impl Counter {
///     pub fn increment(env: Env) -> i32 {
///         // … contract logic …
///         42
///     }
/// }
///
/// #[test]
/// fn measure_increment() {
///     let env = Env::default();
///     let contract_id = env.register(Counter, ());
///     let client = CounterClient::new(&env, &contract_id);
///
///     let value = record(&env, "Counter::increment", || client.increment());
///     assert_eq!(value, 42);
/// }
/// ```
pub fn record<F, R>(env: &Env, label: &str, f: F) -> R
where
    F: FnOnce() -> R,
{
    let mut budget = env.cost_estimate().budget();
    budget.reset_unlimited();

    let result = f();

    let cpu = budget.cpu_instruction_cost();
    let mem = budget.memory_bytes_cost();

    let rec = CostRecord {
        label: label.to_string(),
        cpu_instructions: cpu,
        memory_bytes: mem,
    };

    println!(
        "##SOROBAN_COST_JSON##{}",
        serde_json::to_string(&rec).unwrap()
    );

    result
}
