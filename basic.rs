
---

# Example `contracts/pension_fund/src/lib.rs` (Soroban/Rust skeleton)
> This is a simplified contract skeleton to illustrate functions. Adapt for Soroban's exact API and types as needed.

```rust
#![no_std]

use soroban_sdk::{contractimpl, symbol, Address, Env, Map, Vec, Timestamp, Panic};

pub struct PensionFund;

#[derive(Clone)]
pub struct UserConfig {
    pub retirement_ts: Timestamp,
    pub monthly_percent: u32, // percent of corpus to send monthly (0-100)
    pub corpus: i128, // stored in smallest USDT unit (for demo)
}

#[contractimpl]
impl PensionFund {
    // Initialize (if needed)
    pub fn init(_env: Env) {}

    // Deposit contribution into user's corpus
    pub fn deposit(env: Env, user: Address, amount: i128) {
        let key = (symbol!("usercfg"), user.clone());
        let mut storage: Map<Address, UserConfig> = env.storage().get(&symbol!("users")).unwrap_or_default();
        let mut cfg = storage.get(&user).unwrap_or(UserConfig { retirement_ts: Timestamp::from_unix_timestamp(0), monthly_percent: 0, corpus: 0 });
        cfg.corpus = cfg.corpus + amount;
        storage.set(&user, cfg);
        env.storage().set(&symbol!("users"), &storage);
        // emit Deposit event (left as exercise)
    }

    // Set retirement timestamp and monthly percent
    pub fn set_retirement(env: Env, user: Address, retirement_ts: i64, monthly_percent: u32) {
        assert!(monthly_percent <= 100, "invalid percent");
        let mut storage: Map<Address, UserConfig> = env.storage().get(&symbol!("users")).unwrap_or_default();
        let mut cfg = storage.get(&user).unwrap_or(UserConfig { retirement_ts: Timestamp::from_unix_timestamp(0), monthly_percent: 0, corpus: 0});
        cfg.retirement_ts = Timestamp::from_unix_timestamp(retirement_ts);
        cfg.monthly_percent = monthly_percent;
        storage.set(&user, cfg);
        env.storage().set(&symbol!("users"), &storage);
    }

    // Calculate monthly payout (callable by anyone; actual payment should integrate token transfer)
    pub fn calculate_monthly(env: Env, user: Address) -> i128 {
        let storage: Map<Address, UserConfig> = env.storage().get(&symbol!("users")).unwrap_or_default();
        let cfg = storage.get(&user).expect("user not found");
        let monthly = (cfg.corpus * (cfg.monthly_percent as i128)) / 100;
        monthly
    }

    // Trigger payout (pseudo - actual token move requires token contract)
    pub fn trigger_payout(env: Env, user: Address) {
        let now = env.ledger().timestamp();
        let mut storage: Map<Address, UserConfig> = env.storage().get(&symbol!("users")).unwrap_or_default();
        let mut cfg = storage.get(&user).expect("user not found");
        if now < cfg.retirement_ts {
            panic!("Not retired yet");
        }

        // compute monthly amount
        let monthly = (cfg.corpus * (cfg.monthly_percent as i128)) / 100;
        // Here you would call token contract to transfer `monthly` to user.
        // For demo we simulate by decrementing corpus
        cfg.corpus = cfg.corpus - monthly;
        storage.set(&user, cfg);
        env.storage().set(&symbol!("users"), &storage);
    }

    // Get user config
    pub fn get_user(env: Env, user: Address) -> UserConfig {
        let storage: Map<Address, UserConfig> = env.storage().get(&symbol!("users")).unwrap_or_default();
        storage.get(&user).unwrap_or(UserConfig { retirement_ts: Timestamp::from_unix_timestamp(0), monthly_percent: 0, corpus: 0 })
    }
}
