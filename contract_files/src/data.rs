#![no_std]

use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Contribution(Address),
    TotalContributions,
    Status,
    Disbursed,
    Deadline,
}

#[derive(Clone)]
#[contracttype]
pub enum Status {
    Active,
    Successful,
    Unsuccessful,
}
