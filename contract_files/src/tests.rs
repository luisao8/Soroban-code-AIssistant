```rust
#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as TestAddress, Env};

    #[test]
    fn test_contribute() {
        let env = Env::default();
        let contributor = TestAddress::random(&env);
        let amount = 150;

        CrowdfundingContract::contribute(env.clone(), contributor.clone(), amount);

        let total_contributions: i128 = env.storage().get("total_contributions").unwrap();
        assert_eq!(total_contributions, amount);

        let balance: i128 = env.storage().get(&contributor).unwrap();
        assert_eq!(balance, amount);
    }

    #[test]
    #[should_panic(expected = "Minimum contribution is 100 XLM")]
    fn test_contribute_below_minimum() {
        let env = Env::default();
        let contributor = TestAddress::random(&env);
        let amount = 50;

        CrowdfundingContract::contribute(env, contributor, amount);
    }

    #[test]
    fn test_finalize_campaign_successful() {
        let env = Env::default();
        let recipient = TestAddress::random(&env);
        env.storage().set("funding_goal", 1000);
        env.storage().set("total_contributions", 1500);
        env.storage().set("deadline", env.ledger().timestamp() - 1);
        env.storage().set("recipient", recipient.clone());

        CrowdfundingContract::finalize_campaign(env.clone());

        let campaign_status: String = env.storage().get("campaign_status").unwrap();
        assert_eq!(campaign_status, "successful");
    }

    #[test]
    fn test_finalize_campaign_unsuccessful() {
        let env = Env::default();
        env.storage().set("funding_goal", 1000);
        env.storage().set("total_contributions", 500);
        env.storage().set("deadline", env.ledger().timestamp() - 1);

        CrowdfundingContract::finalize_campaign(env.clone());

        let campaign_status: String = env.storage().get("campaign_status").unwrap();
        assert_eq!(campaign_status, "unsuccessful");
    }

    #[test]
    fn test_claim_refund() {
        let env = Env::default();
        let contributor = TestAddress::random(&env);
        env.storage().set("campaign_status", "unsuccessful");
        env.storage().set(&contributor, 200);

        CrowdfundingContract::claim_refund(env.clone(), contributor.clone());

        let refund_status: bool = env.storage().get(&contributor).unwrap();
        assert!(refund_status);
    }

    #[test]
    #[should_panic(expected = "Refunds are not available")]
    fn test_claim_refund_not_available() {
        let env = Env::default();
        let contributor = TestAddress::random(&env);
        env.storage().set("campaign_status", "successful");

        CrowdfundingContract::claim_refund(env, contributor);
    }

    #[test]
    #[should_panic(expected = "Unauthorized access")]
    fn test_authorize_unauthorized() {
        let env = Env::default();
        let unauthorized_address = TestAddress::random(&env);

        CrowdfundingContract::authorize(env, unauthorized_address);
    }
}
```