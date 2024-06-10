```rust
use soroban_sdk::{Address, Env};

pub fn authorize(env: Env, address: Address) {
    let authorized_addresses = env.storage().get::<Vec<Address>>("authorized_addresses").unwrap();
    if !authorized_addresses.contains(&address) {
        panic!("Unauthorized access");
    }
}
```