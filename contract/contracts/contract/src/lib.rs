#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Address};

#[contract]   // ✅ REQUIRED
pub struct FarmingContract;

#[derive(Clone)]
#[contracttype]
pub struct Farmer {
    pub staked_amount: i128,
    pub reward_debt: i128,
}

#[contracttype]
pub enum DataKey {
    Farmer(Address),
    TotalStaked,
    RewardRate,
}

#[contractimpl]
impl FarmingContract {

    pub fn init(env: Env, reward_rate: i128) {
        env.storage().instance().set(&DataKey::RewardRate, &reward_rate);
        env.storage().instance().set(&DataKey::TotalStaked, &0i128);
    }

    pub fn stake(env: Env, user: Address, amount: i128) {
        user.require_auth();

        let mut farmer = env.storage()
            .instance()
            .get(&DataKey::Farmer(user.clone()))
            .unwrap_or(Farmer {
                staked_amount: 0,
                reward_debt: 0,
            });

        farmer.staked_amount += amount;

        let mut total: i128 = env.storage()
            .instance()
            .get(&DataKey::TotalStaked)
            .unwrap_or(0);

        total += amount;

        env.storage().instance().set(&DataKey::Farmer(user), &farmer);
        env.storage().instance().set(&DataKey::TotalStaked, &total);
    }
}