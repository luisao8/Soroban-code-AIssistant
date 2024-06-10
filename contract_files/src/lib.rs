#![no_std]

extern crate alloc;

use soroban_sdk::{contract, contractimpl, Address, Env};

mod contribution;
mod finalization;
mod disbursement;
mod refund;
mod security;
mod data;

#[contract]
pub struct CrowdfundingContract;

#[contractimpl]
impl CrowdfundingContract {
    pub fn contribute(env: Env, from: Address, amount: i128) {
        contribution::Contribution::contribute(env, from, amount);
    }

    pub fn finalize(env: Env, admin: Address) {
        finalization::Finalization::finalize(env, admin);
    }

    pub fn disburse(env: Env, admin: Address, recipient: Address) {
        disbursement::Disbursement::disburse(env, admin, recipient);
    }

    pub fn refund(env: Env, to: Address) {
        refund::Refund::refund(env, to);
    }
}

mod test;
