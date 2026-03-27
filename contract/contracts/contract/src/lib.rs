#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, symbol_short, Symbol};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Status {
    Open,
    Resolved,
}

#[contracttype]
#[derive(Clone)]
pub struct Bounty {
    pub creator: Address,
    pub description: String,
    pub reward_amount: u32,
    pub status: Status,
    pub solver: Option<Address>,
}

const BOUNTY_COUNT: Symbol = symbol_short!("COUNT");

#[contract]
pub struct BugBountyTracker;

#[contractimpl]
impl BugBountyTracker {
    fn get_count(env: &Env) -> u32 {
        env.storage().instance().get(&BOUNTY_COUNT).unwrap_or(0)
    }

    pub fn create_bounty(env: Env, creator: Address, description: String, reward_amount: u32) -> u32 {
        creator.require_auth();
        let mut count = Self::get_count(&env);
        count += 1;

        let bounty = Bounty {
            creator,
            description,
            reward_amount,
            status: Status::Open,
            solver: None,
        };

        env.storage().persistent().set(&count, &bounty);
        env.storage().instance().set(&BOUNTY_COUNT, &count);
        count
    }

    pub fn resolve_bounty(env: Env, bounty_id: u32, solver: Address) {
        let mut bounty: Bounty = env.storage().persistent().get(&bounty_id).expect("Bounty not found");
        bounty.creator.require_auth();

        if bounty.status == Status::Resolved {
            panic!("Bounty is already resolved");
        }

        bounty.status = Status::Resolved;
        bounty.solver = Some(solver);
        env.storage().persistent().set(&bounty_id, &bounty);
    }

    pub fn get_bounty(env: Env, bounty_id: u32) -> Bounty {
        env.storage().persistent().get(&bounty_id).expect("Bounty not found")
    }
}