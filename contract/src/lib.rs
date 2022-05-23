/*
 * This is an example of a Rust smart contract with two simple, symmetric functions:
 *
 * 1. set_greeting: accepts a greeting, such as "howdy", and records it for the user (account_id)
 *    who sent the request
 * 2. get_greeting: accepts an account_id and returns the greeting saved for it, defaulting to
 *    "Hello"
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://github.com/near/near-sdk-rs
 *
 */

mod models;
mod utils;
use crate::{
    models::{Crowdfund, Donation},
    utils::{assert_self, assert_single_promise_success, AccountId, ONE_NEAR},
};

// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use near_sdk::{env, near_bindgen, PromiseIndex};
near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(Clone, Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    owner: AccountId,
    crowdfunds: Vec<Crowdfund>,
    donations: Vec<Donation>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn init(owner: AccountId) -> Self {
        let crowdfunds: Vec<Crowdfund> = Vec::new();
        let donations: Vec<Donation> = Vec::new();

        Contract {
            owner,
            crowdfunds,
            donations,
        }
    }

    pub fn add_crowdfund(&mut self, title: String, donate: u128, description: String) {
        let id = self.crowdfunds.len() as i32;
        self.crowdfunds
            .push(Crowdfund::new(id, title, donate, description));
        env::log("Added a new crowdfund".as_bytes());
    }

    pub fn list_crowdfunds(&self) -> Vec<Crowdfund> {
        // assert_self();
        let crowdfunds = &self.crowdfunds;
        return crowdfunds.to_vec();
    }

    pub fn add_vote(&mut self, id: usize) {
        let crowdfund: &mut Crowdfund = self.crowdfunds.get_mut(id).unwrap();
        let voter = env::predecessor_account_id();
        crowdfund.total_votes += 1;
        env::log("vote submitted succesfully".as_bytes());
        crowdfund.votes.push(voter);
    }

    // maybe not need to pass amount, just use env::attached_deposit instead
    pub fn add_donation(&mut self, id: usize, amount: u128) {
        let transfer_amount: u128 = ONE_NEAR * amount;
        let crowdfund: &mut Crowdfund = self.crowdfunds.get_mut(id).unwrap();
        crowdfund.total_donations = crowdfund.total_donations + transfer_amount;
        self.donations.push(Donation::new());

        println!("transfer_amount: {}", transfer_amount);
        println!("amount: {}", amount);
        println!("attached_deposit: {}", env::attached_deposit());
        near_sdk::Promise::new(env::predecessor_account_id()).transfer(transfer_amount);
        env::log("You have donated succesfully".as_bytes());
    }

    pub fn crowdfund_count(&mut self) -> usize {
        return self.crowdfunds.len();
    }

    pub fn get_total_donations(&mut self, id: usize) -> u128 {
        let crowdfund: &mut Crowdfund = self.crowdfunds.get_mut(id).unwrap();
        return crowdfund.total_donations;
    }
}

// near call crowdfunddapp.verkhohliad.testnet add_crowdfund '{"title": "Eliots eye sight", "donate": 30, "description":"Raise funds for little Eliot to see again. Loss of sight was caused by an accident to the head"}' --accountId verkhohliad.testnet

// near call crowdfunddapp.verkhohliad.testnet add_vote '{"id":0}' --accountId verkhohliad.testnet

// near call crowdfunddapp.verkhohliad.testnet add_donation '{"id":0, "amount":1}' --accountId verkhohliad.testnet

// near call crowdfunddapp.verkhohliad.testnet list_crowdfunds --accountId verkhohliad.testnet

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 *
 * To run from contract directory:
 * cargo test -- --nocapture
 *
 * From project root, to run in combination with frontend tests:
 * yarn test
 *
 */
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // mock the context for testing, notice "signer_account_id" that was accessed above from env::
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn set_then_get_greeting() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Welcome::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(
            "howdy".to_string(),
            contract.get_greeting("bob_near".to_string())
        );
    }

    #[test]
    fn get_default_greeting() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let contract = Welcome::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(
            "Hello".to_string(),
            contract.get_greeting("francis.near".to_string())
        );
    }
}
