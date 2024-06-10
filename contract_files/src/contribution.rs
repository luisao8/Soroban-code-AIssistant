```rust
use soroban_sdk::{Address, Env};

pub fn contribute(env: Env, contributor: Address, amount: i128) {
    if amount < 100 {
        panic!("Minimum contribution is 100 XLM");
    }

    let total_contributions = env.storage().get::<i128>("total_contributions").unwrap_or(0);
    env.storage().set("total_contributions", total_contributions + amount);

    env.events().publish((contributor.clone(), amount, env.ledger().timestamp()));

    let balance = env.storage().get::<i128>(&contributor).unwrap_or(0);
    env.storage().set(&contributor, balance + amount);
}
```