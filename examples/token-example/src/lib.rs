#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Vec};

#[contracttype]
pub enum DataKey {
    Admin,
    Balance(Address),
}

#[contract]
pub struct Token;

#[contractimpl]
impl Token {
    /// Initialize the token with an admin address.
    pub fn initialize(env: Env, admin: Address) {
        env.storage().persistent().set(&DataKey::Admin, &admin);
    }

    /// Mint `amount` tokens to `to`.
    pub fn mint(env: Env, to: Address, amount: i128) {
        let key = DataKey::Balance(to.clone());
        let current: i128 = env.storage().persistent().get(&key).unwrap_or(0);
        env.storage().persistent().set(&key, &(current + amount));
    }

    /// Transfer `amount` tokens from `from` to `to`.
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        let from_key = DataKey::Balance(from.clone());
        let to_key = DataKey::Balance(to.clone());

        let from_balance: i128 = env.storage().persistent().get(&from_key).unwrap_or(0);
        let to_balance: i128 = env.storage().persistent().get(&to_key).unwrap_or(0);

        env.storage().persistent().set(&from_key, &(from_balance - amount));
        env.storage().persistent().set(&to_key, &(to_balance + amount));
    }

    /// Swap: transfer `amount_a` of token A from `user` to `to`,
    /// and `amount_b` of token B from `to` to `user`.
    /// Simplified — uses two different storage keys per user to simulate two tokens.
    pub fn swap(
        env: Env,
        user: Address,
        to: Address,
        amount_a: i128,
        amount_b: i128,
    ) {
        // Read user's A balance
        let user_a_key = DataKey::Balance(user.clone());
        let user_a: i128 = env.storage().persistent().get(&user_a_key).unwrap_or(0);

        // Read to's B balance
        let to_b_key = DataKey::Balance(to.clone());
        let to_b: i128 = env.storage().persistent().get(&to_b_key).unwrap_or(0);

        // Write updated balances
        env.storage()
            .persistent()
            .set(&user_a_key, &(user_a - amount_a));
        env.storage()
            .persistent()
            .set(&to_b_key, &(to_b + amount_b));
    }

    /// Transfer `amount` tokens from admin to each address in `recipients`.
    ///
    /// DELIBERATELY INEFFICIENT: reads the admin address from storage once per
    /// recipient instead of caching it before the loop. This makes it the worst
    /// offender in cost reports — exactly what we want for profiling.
    pub fn batch_transfer(env: Env, recipients: Vec<Address>, amount: i128) {
        for recipient in recipients.iter() {
            // BAD: re-reads the admin key every iteration instead of caching it.
            let admin: Address = env.storage().persistent().get(&DataKey::Admin).unwrap();
            let key = DataKey::Balance(recipient.clone());
            let current: i128 = env.storage().persistent().get(&key).unwrap_or(0);
            env.storage().persistent().set(&key, &(current + amount));
            // The admin balance should decrease but we skip that for simplicity.
            let _ = admin;
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use soroban_sdk::testutils::Address as _;
    use soroban_cost_harness::record;

    use super::*;

    fn setup() -> (Env, Address, Address, Address, Address) {
        let env = Env::default();
        let admin = Address::generate(&env);
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);

        env.mock_all_auths();

        let contract_id = env.register(Token, ());
        let client = TokenClient::new(&env, &contract_id);

        client.initialize(&admin);
        client.mint(&user1, &1_000_000);
        client.mint(&user2, &500_000);

        (env, contract_id, admin, user1, user2)
    }

    #[test]
    fn measure_transfer() {
        let (env, contract_id, _admin, user1, user2) = setup();
        let client = TokenClient::new(&env, &contract_id);

        record(&env, "transfer", || {
            client.transfer(&user1, &user2, &100);
        });
    }

    #[test]
    fn measure_mint() {
        let (env, contract_id, _admin, _user1, _user2) = setup();
        let client = TokenClient::new(&env, &contract_id);
        let recipient = Address::generate(&env);

        record(&env, "mint", || {
            client.mint(&recipient, &500);
        });
    }

    #[test]
    fn measure_swap() {
        let (env, contract_id, _admin, user1, user2) = setup();
        let client = TokenClient::new(&env, &contract_id);

        record(&env, "swap", || {
            client.swap(&user1, &user2, &50, &30);
        });
    }

    #[test]
    fn measure_batch_transfer() {
        let (env, contract_id, admin, _user1, _user2) = setup();
        let client = TokenClient::new(&env, &contract_id);

        // Mint a big balance to admin so batch_transfer has enough.
        client.mint(&admin, &10_000_000);

        let recipients = soroban_sdk::vec![
            &env,
            Address::generate(&env),
            Address::generate(&env),
            Address::generate(&env),
            Address::generate(&env),
            Address::generate(&env),
        ];

        record(&env, "batch_transfer", || {
            client.batch_transfer(&recipients, &100);
        });
    }
}
