```rust
use soroban_sdk::{Address};

#[derive(Clone)]
pub struct Contribution {
    pub contributor: Address,
    pub amount: i128,
    pub timestamp: u64,
}

#[derive(Clone)]
pub struct Campaign {
    pub funding_goal: i128,
    pub deadline: u64,
    pub recipient: Address,
    pub total_contributions: i128,
    pub status: CampaignStatus,
}

#[derive(Clone, PartialEq)]
pub enum CampaignStatus {
    Active,
    Successful,
    Unsuccessful,
}
```