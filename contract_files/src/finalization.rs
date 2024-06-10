#![no_std]

use soroban_sdk::{contractimpl, Address, Env};

pub struct Finalization;

#[contractimpl]
impl Finalization {
    pub fn finalize(env: Env, admin: Address) {
        admin.require_auth();
        let current_time = env.ledger().timestamp();
        let deadline: u64 = env.storage().persistent().get(&DataKey::Deadline).unwrap();
        if current_time < deadline {
            panic!("Cannot finalize before the campaign deadline");
        }
        let total: i128 = env.storage().persistent().get(&DataKey::TotalContributions).unwrap_or(0);
        let status = if total >= 50_000 {
            Status::Successful
        } else {
            Status::Unsuccessful
        };
        env.storage().persistent().set(&DataKey::Status, &status);
        // Log event
        env.events().publish((admin, status), "CampaignFinalized");
    }
}
