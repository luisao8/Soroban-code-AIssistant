#![no_std]

use soroban_sdk::{contractimpl, Address, Env};

pub struct Disbursement;

#[contractimpl]
impl Disbursement {
    pub fn disburse(env: Env, admin: Address, recipient: Address) {
        admin.require_auth();
        let status: Status = env.storage().persistent().get(&DataKey::Status).unwrap();
        if status != Status::Successful {
            panic!("Campaign not successful");
        }
        let disbursed: bool = env.storage().persistent().get(&DataKey::Disbursed).unwrap_or(false);
        if disbursed {
            panic!("Funds have already been disbursed");
        }
        let total: i128 = env.storage().persistent().get(&DataKey::TotalContributions).unwrap();
        env.payments().pay(recipient.clone(), total);
        env.storage().persistent().set(&DataKey::Disbursed, &true);
        // Log event
        env.events().publish((admin, recipient, total), "FundsDisbursed");
    }
}
