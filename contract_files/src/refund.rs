```rust
use soroban_sdk::{Address, Env};
use crate::integration::token;

pub fn claim_refund(env: Env, contributor: Address) {
    let campaign_status = env.storage().get::<String>("campaign_status").unwrap();
    if campaign_status != "unsuccessful" {
        panic!("Refunds are not available");
    }

    let refund_status = env.storage().get::<bool>(&contributor).unwrap_or(false);
    if refund_status {
        panic!("Refund already claimed");
    }

    let balance = env.storage().get::<i128>(&contributor).unwrap_or(0);
    token::Client::new(&env, &token::TOKEN_ADDRESS).transfer(&env.current_contract_address(), &contributor, &balance);

    env.storage().set(&contributor, true);
}
```