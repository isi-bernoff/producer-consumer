use std::collections::HashMap;
use std::thread::Thread;
use std::sync::Mutex;
use std::fmt::{Formatter, Display, Result};

enum Mode {
    Deposit,
    Withdraw,
    Transfer
}

struct Transaction {
    from_id: u16,
    to_id: u16,
    amount: i32,
    mode: Mode
}

pub struct Bank {
    threads: HashMap<u16, Thread>,
    accounts: HashMap<u16, Mutex<i32>>,
    ledger: HashMap<u16, Transaction>,
    num_successes: u16,
    num_failures: u16
}

impl Bank {
    /// Constructs a new `Bank` object and initializes its `accounts`
    /// #### Parameters:
    /// `num_accounts` &ndash; The number of `Account`s to initialize
    /// ### Returns:
    /// The new `Bank` object
    pub fn new(num_accounts: u16) -> Self {
        todo!();
    }
    
    /// Initializes this `Bank`'s `threads` and `ledger`
    /// #### Parameters:
    /// `num_threads` &ndash; The number of `Thread`s to initialize \
    /// `ledger_filename` &ndash; The name of a ledger file containing
    /// transactions formatted `<from_id> <to_id> <amount> <mode>` on each line
    pub fn init(num_threads: u16, ledger_filename: String) {
        todo!();
    }
    
    /// Deposit money into one of this `Bank`'s `accounts`, incrementing
    /// `num_successes` or `num_failures` depending on \
    /// if the deposit works
    /// #### Parameters:
    /// `thread_id` &ndash; The identifier of the `Thread` performing the the
    /// deposit \
    /// `deposit_id` &ndash; The identifier of the deposit `Transaction` \
    /// `account_id` &ndash; The identifier of the account receiving the
    /// deposit \
    /// `amount` &ndash; The amount of money being deposited
    /// #### Returns:
    /// A success message if the deposit succeeds and a failure message
    /// otherwise
    pub fn deposit(thread_id: u16, deposit_id: u16, account_id: u16,
                   amount: i32) -> String {
        todo!();
    }

    /// Withdraws money from one of this `Bank`'s `accounts`, incrementing
    /// `num_successes` or `num_failures` depending on \
    /// if the withdrawal works
    /// #### Parameters:
    /// `thread_id` &ndash; The identifier of the `Thread` performing the the
    /// withdrawal \
    /// `withdrawal_id` &ndash; The identifier of the withdrawal `Transaction` \
    /// `account_id` &ndash; The identifier of the account having the
    /// withdrawal \
    /// `amount` &ndash; The amount of money being withdrawn
    /// #### Returns:
    /// A success message if the withdrawal succeeds and a failure message
    /// otherwise
    pub fn withdraw(thread_id: u16, withdrawal_id: u16, account_id: u16,
        amount: i32) -> String {
        todo!();
    }

    /// Transfers money from one of this `Bank`'s `accounts` to another,
    /// incrementing `num_successes` or `num_failures` \
    /// depending on if the transfer works
    /// #### Parameters:
    /// `thread_id` &ndash; The identifier of the `Thread` performing the the
    /// transfer \
    /// `transfer_id` &ndash; The identifier of the transfer `Transaction` \
    /// `from_id` &ndash; The identifier of the account having money transferred
    /// from it \
    /// `to_id` &ndash; The identifier of the account having money transferred
    /// to it \
    /// `amount` &ndash; The amount of money being withdrawn
    /// #### Returns:
    /// A success message if the transfer succeeds and a failure message
    /// otherwise
    pub fn transfer(thread_id: u16, transfer: u16, from_id: u16,
        to_id: u16, amount: i32) -> String {
        todo!();
    }

    /// Pops a `Transaction` from the `Bank`'s `ledger` and uses a `Thread`
    /// to perform it
    /// concurrently
    /// #### Parameters:
    /// `id` &ndash; The identifier of the `Thread` performing the `Transaction`
    pub fn init_thread(id: u16) {
        todo!();
    }
}

impl Drop for Bank {
    fn drop(&mut self) {
        todo!();
    }
}

impl Display for Bank {
    fn fmt (&self, f: &mut Formatter<'_>) -> Result {
        todo!();
    }
}