```rust
extern crate alloc;

use soroban_sdk::{contractimpl, Address, Env};

mod contribution;
mod finalization;
mod refund;
mod security;
mod integration;
mod types;

pub struct CrowdfundingContract;

#[contractimpl]
impl CrowdfundingContract {
    pub fn contribute(env: Env, contributor: Address, amount: i128) {
        contribution::contribute(env, contributor, amount);
    }

    pub fn finalize_campaign(env: Env) {
        finalization::finalize_campaign(env);
    }

    pub fn claim_refund(env: Env, contributor: Address) {
        refund::claim_refund(env, contributor);
    }

    pub fn authorize(env: Env, address: Address) {
        security::authorize(env, address);
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        integration::transfer(env, from, to, amount);
    }

    pub fn log_event(env: Env, event: String) {
        integration::log_event(env, event);
    }
}

#[cfg(test)]
mod tests;
```