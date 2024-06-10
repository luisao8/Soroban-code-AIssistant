#![no_std]

use soroban_sdk::{contractimpl, Address, Env};

pub struct Contribution;

#[contractimpl]
impl Contribution {
    pub fn contribute(env: Env, from: Address, amount: i128) {
        from.require_auth();
        if amount <= 0 {
            panic!("Contribution amount must be greater than zero");
        }
        // Check-Effects-Interactions pattern
        let total: i128 = env.storage().persistent().get(&DataKey::TotalContributions).unwrap_or(0);
        env.storage().persistent().set(&DataKey::TotalContributions, &(total + amount));
        // Log the contribution
        env.storage().persistent().set(&DataKey::Contribution(from.clone()), &amount);
        env.payments().pay(env.current_contract_address(), amount);
        // Log event
        env.events().publish((from, amount), "ContributionReceived");
    }
}
