```rust
use soroban_sdk::{Address, Env};
use crate::integration::token;

pub fn finalize_campaign(env: Env) {
    let deadline = env.storage().get::<u64>("deadline").unwrap();
    if env.ledger().timestamp() < deadline {
        panic!("Campaign is still active");
    }

    let total_contributions = env.storage().get::<i128>("total_contributions").unwrap_or(0);
    let funding_goal = env.storage().get::<i128>("funding_goal").unwrap();
    if total_contributions >= funding_goal {
        let recipient = env.storage().get::<Address>("recipient").unwrap();
        token::Client::new(&env, &token::TOKEN_ADDRESS).transfer(&env.current_contract_address(), &recipient, &total_contributions);
        env.storage().set("campaign_status", "successful");
    } else {
        env.storage().set("campaign_status", "unsuccessful");
    }
}
```