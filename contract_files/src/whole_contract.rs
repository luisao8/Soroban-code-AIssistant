#![no_std]

extern crate alloc;

use soroban_sdk::{contractimpl, Address, Env, contracttype, ConversionError, Val, TryFromVal};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Contribution(Address),
    TotalContributions,
    Status,
    Disbursed,
    Deadline,
    FundingGoal,
    Recipient,
    TokenContract,
}

#[derive(Clone)]
#[contracttype]
pub enum Status {
    Active,
    Successful,
    Unsuccessful,
}

impl TryFromVal<Env, DataKey> for Val {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &DataKey) -> Result<Self, Self::Error> {
        Ok((*v as u32).into())
    }
}

pub struct CrowdfundingContract;

#[contractimpl]
impl CrowdfundingContract {
    pub fn init(env: Env, admin: Address, funding_goal: i128, deadline: u64, recipient: Address) {
        admin.require_auth();
        env.storage().persistent().set(&DataKey::FundingGoal, &funding_goal);
        env.storage().persistent().set(&DataKey::Deadline, &deadline);
        env.storage().persistent().set(&DataKey::Recipient, &recipient);
        env.storage().persistent().set(&DataKey::TotalContributions, &0_i128);
        env.storage().persistent().set(&DataKey::Status, &Status::Active);
        env.storage().persistent().set(&DataKey::Disbursed, &false);
        env.events().publish((admin, funding_goal, deadline, recipient.clone()), "CampaignInitialized");
    }

    pub fn contribute(env: Env, from: Address, amount: i128) {
        from.require_auth();
        if amount <= 0 {
            panic!("Contribution amount must be greater than zero");
        }
        let mut total: i128 = env.storage().persistent().get(&DataKey::TotalContributions).unwrap_or(0);
        total += amount;
        env.storage().persistent().set(&DataKey::TotalContributions, &total);
        let contribution: i128 = env.storage().persistent().get(&DataKey::Contribution(from.clone())).unwrap_or(0);
        env.storage().persistent().set(&DataKey::Contribution(from.clone()), &(contribution + amount));
        env.payments().pay(env.current_contract_address(), amount);
        env.events().publish((from, amount), "ContributionReceived");
    }

    pub fn finalize(env: Env, admin: Address) {
        admin.require_auth();
        let current_time = env.ledger().timestamp();
        let deadline: u64 = env.storage().persistent().get(&DataKey::Deadline).unwrap();
        if current_time < deadline {
            panic!("Cannot finalize before the campaign deadline");
        }
        let total: i128 = env.storage().persistent().get(&DataKey::TotalContributions).unwrap_or(0);
        let funding_goal: i128 = env.storage().persistent().get(&DataKey::FundingGoal).unwrap();
        let status = if total >= funding_goal {
            Status::Successful
        } else {
            Status::Unsuccessful
        };
        env.storage().persistent().set(&DataKey::Status, &status);
        env.events().publish((admin, status), "CampaignFinalized");
    }

    pub fn disburse(env: Env, admin: Address) {
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
        let recipient: Address = env.storage().persistent().get(&DataKey::Recipient).unwrap();
        env.payments().pay(recipient.clone(), total);
        env.storage().persistent().set(&DataKey::Disbursed, &true);
        env.events().publish((admin, recipient, total), "FundsDisbursed");
    }

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
            env.events().publish((to, amount), "RefundProcessed");
        } else {
            panic!("No contribution found for refund");
        }
    }

    pub fn set_token_contract(env: Env, admin: Address, token_address: Address) {
        admin.require_auth();
        env.storage().persistent().set(&DataKey::TokenContract, &token_address);
    }
}

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
