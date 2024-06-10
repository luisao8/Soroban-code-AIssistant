#![no_std]

use soroban_sdk::{Address, Env};

pub struct Security;

impl Security {
    pub fn require_admin(env: &Env, admin: Address) {
        admin.require_auth();
    }

    pub fn validate_contribution(amount: i128) {
        if amount <= 0 {
            panic!("Invalid contribution amount");
        }
    }
}
