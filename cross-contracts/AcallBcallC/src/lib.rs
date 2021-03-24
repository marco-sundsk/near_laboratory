/*
 * This is contract A:
 * 
 * 
 */

// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::wee_alloc;
use near_sdk::{env, near_bindgen, Promise, ext_contract, Gas, Balance, AccountId};

const DEPOSIT_FIVE_NEAR: Balance = 5_000_000_000_000_000_000_000_000;
const GAS_FOR_BASIC: Gas = 5_000_000_000_000;


#[ext_contract(ext_cb)]
trait Other {
    fn call_has_promise(&mut self) -> Promise;
    fn call_no_promise(&mut self) -> u8;
}



#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub contract_next: AccountId,
    pub status: u8,
}

impl Default for Contract {
    fn default() -> Self {
        env::panic(b"This contract should be initialized before usage")
    }
}

#[near_bindgen]
impl Contract {

    #[init]
    pub fn new(contract_next: AccountId, status: u8,) -> Self {
        assert!(!env::state_exists(), "Already initialized");

        if !env::is_valid_account_id(contract_next.as_bytes()) {
            env::log(format!("This is {}::new, contract_next {} is invalid, set to the end of crosscall",
                env::current_account_id(), 
                contract_next).as_bytes()
            );
        }
        
        Self {
            contract_next,
            status,
        }
    }

    pub fn get_status(&self) -> u8 {
        self.status
    }

    /// return prev status
    pub fn set_status(&mut self, status: u8) -> u8 {
        let ret = self.status;
        self.status = status;
        ret
    }

    #[payable]
    pub fn call_has_promise(&mut self) -> Promise {

        env::log(format!("This is {}::call_has_promise called by {}, prapaid_gas {} ",
            env::current_account_id(), 
            env::predecessor_account_id(), 
            env::prepaid_gas()).as_bytes()
        );

        let amount = env::attached_deposit();
        assert!(amount > DEPOSIT_FIVE_NEAR, "The deposit should more than 5 near.");

        self.status = 8_u8;

        if env::is_valid_account_id(self.contract_next.as_bytes()) {
            ext_cb::call_has_promise(
                &self.contract_next,
                amount - DEPOSIT_FIVE_NEAR,
                env::prepaid_gas() - GAS_FOR_BASIC,
            )
        } else {
            env::log(format!("This is the end of crosscall link.").as_bytes());
            Promise::new(env::signer_account_id()).transfer(10)
        }        
    }

    #[payable]
    pub fn call_no_promise(&mut self) {

        env::log(format!("This is {}::call_has_promise called by {}, prapaid_gas {} ",
            env::current_account_id(), 
            env::predecessor_account_id(), 
            env::prepaid_gas()).as_bytes()
        );

        let amount = env::attached_deposit();
        assert!(amount > DEPOSIT_FIVE_NEAR, "The deposit should more than 5 near.");

        self.status = 8_u8;

        ext_cb::call_has_promise(
            &self.contract_next,
            amount - DEPOSIT_FIVE_NEAR,
            env::prepaid_gas() - GAS_FOR_BASIC,
        );
        if env::is_valid_account_id(self.contract_next.as_bytes()) {
            ext_cb::call_no_promise(
                &self.contract_next,
                amount - DEPOSIT_FIVE_NEAR,
                env::prepaid_gas() - GAS_FOR_BASIC,
            );
        } else {
            env::log(format!("This is the end of crosscall link.").as_bytes());
        }  
    }

}

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
    }

    #[test]
    fn get_default_greeting() {
        let context = get_context(vec![], true);
        testing_env!(context);
        // this test did not call set_greeting so should return the default "Hello" greeting
    }
}
