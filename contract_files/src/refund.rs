#![no_std]

use soroban_sdk::{contractimpl, Address, Env};

pub struct Refund;

#[contractimpl]
impl Refund {
    pub fn refund(env: Env, to: Address) {
        to.require_auth();
        let status: Status = env.storage().persistent().get(&DataKey::Status).unwrap();
        if status != Status::Unsuccessful {
            panic!("Campaign not unsuccessful");
        }
        let amount: i128 = env.storage().persistent().get(&DataKey::Contribution(to.clone())).unwrap_or(0);
        if amount > 0 {
            env.payments().pay(to.clone(), amount);
            env.storage().persistent().remove(&DataKey::Contribution(to));
            // Log event
            env.events().publish((to, amount), "RefundProcessed");
        } else {
            panic!("No contribution found for refund");
        }
    }
}
