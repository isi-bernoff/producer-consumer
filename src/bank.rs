use std::collections::HashMap;
use std::thread::{JoinHandle, spawn};
use std::sync::Mutex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use self::transaction::Transaction;
mod transaction;

/// #### Fields:
/// - `join_handles`: A `HashMap` of `u16` identifiers to `String` messages returned from
///   `Threads`s' `JoinHandle`s
/// - `accounts`: A `HashMap` of `u16` identifiers to `Mutex`-locked `f32` balances
/// - `ledger`: A `HashMap` of `u16` identifiers to `Transactions`
/// - `num_successes`: The `u16` number of `Transaction`s that succeeded
/// - `num_failures`: The `u16` number of `Transaction`s that failed
pub struct Bank {
    join_handles: HashMap<u16, JoinHandle<String>>,
    accounts: HashMap<u16, Mutex<f32>>,
    ledger: HashMap<u16, Transaction>,
    num_successes: u16,
    num_failures: u16
}

impl Bank {
    /// Constructs a new `Bank` object and initializes its `accounts`, `ledger`, `num_successes`,
    /// and `num_failures`
    /// #### Parameters
    /// - `num_accounts`: The number of `account`s to initialize
    /// - `ledger_filepath`: The name of a ledger file containing transactions formatted `<from_id>
    ///   <to_id> <amount> <mode_num>` on each line
    /// #### Returns
    /// The new `Bank` object
    pub fn new(num_accounts: u16, ledger_filepath: String) -> Self {
        let join_handles: HashMap<u16, JoinHandle<String>> = HashMap::new();
        let mut accounts: HashMap<u16, Mutex<f32>> = HashMap::new();
        let mut ledger: HashMap<u16, Transaction> = HashMap::new();
        let num_successes: u16 = 0;
        let num_failures: u16 = 0;

        for id in 0u16..num_accounts {
            let mutex: Mutex<f32> = Mutex::new(0.0);
            accounts.insert(id, mutex);
        }

        let file: File = File::open(ledger_filepath).unwrap();
        let reader: BufReader<File> = BufReader::new(file);
        
        for (id, line) in reader.lines().enumerate() {
            let id: u16 = u16::try_from(id).unwrap();
            let line: String = line.unwrap();
            let mut tokens = line.splitn(4, ' ');
            let from_id: u16 = tokens.next().unwrap().parse::<u16>().unwrap();
            let to_id: u16 = tokens.next().unwrap().parse::<u16>().unwrap();
            let amount: f32 = tokens.next().unwrap().parse::<f32>().unwrap();
            let mode_id: u8 = tokens.next().unwrap().parse::<u8>().unwrap();
            let transaction: Transaction = Transaction::new(from_id, to_id, amount, mode_id);
            ledger.insert(id, transaction);
        }

        Self {join_handles, accounts, ledger, num_successes, num_failures}
    }
    
    /// Spawns `Threads` to initialize this `Bank`'s `join_handles`
    /// #### Parameters
    /// - `num_threads`: The number of `Thread`s to spawn
    pub fn spawn(&mut self, num_threads: u16) {
        for id in 0u16..num_threads {
            let join_handle: JoinHandle<String> = spawn(move || {
                let thread_id: u16 = id;
                return Self::perform_transaction(thread_id);
            });
            self.join_handles.insert(id, join_handle);
        }
    }

    /// Pops a `Transaction` from the `Bank`'s ledger and uses a `Thread`
    /// to process it concurrently
    /// #### Parameters
    /// `thread_id`: The identifier of the thread processing the `Transaction`
    /// #### Returns
    /// A success message if the `Transaction` succeeds and a failure message otherwise
    pub fn perform_transaction(thread_id: u16) -> String {
        todo!();
    }
    
    /// Deposit money into one of this `Bank`'s `accounts`, incrementing
    /// `num_successes` or `num_failures` depending on if the deposit works
    /// #### Parameters
    /// - `thread_id`: The identifier of the `Thread` processing the the deposit
    /// - `transaction_id`: The identifier of the deposit `Transaction`
    /// - `account_id`: The identifier of the account receiving the deposit
    /// - `amount`: The amount of money being deposited
    /// #### Returns
    /// `true` if the deposit succeeds and `false` otherwise
    pub fn deposit(&self, thread_id: u16, transaction_id: u16, account_id: u16,
                   amount: f32) -> bool {
        todo!();
    }

    /// Withdraws money from one of this `Bank`'s `accounts`, incrementing
    /// `num_successes` or `num_failures` depending on if the withdrawal works
    /// #### Parameters
    /// - `thread_id`: The identifier of the `Thread` processing the the withdrawal
    /// - `transaction_id`: The identifier of the withdrawal `Transaction`
    /// - `account_id`: The identifier of the account having the withdrawal
    /// - `amount`: The amount of money being withdrawn
    /// #### Returns
    /// `true` if the withdrawal succeeds and `false` otherwise
    pub fn withdraw(&self, thread_id: u16, transaction_id: u16, account_id: u16,
        amount: f32) -> bool {
        todo!();
    }

    /// Transfers money from one of this `Bank`'s `accounts` to another,
    /// incrementing `num_successes` or `num_failures` depending on if the
    /// transfer works
    /// #### Parameters
    /// - `thread_id`: The identifier of the `Thread` processing the the transfer
    /// - `transaction_id`: The identifier of the transfer `Transaction`
    /// - `from_id`: The identifier of the account having money transferred from it
    /// - `to_id`: The identifier of the account having money transferred to it
    /// - `amount`: The amount of money being withdrawn
    /// #### Returns
    /// `true` if the transfer succeeds and `false` otherwise
    pub fn transfer(&self, thread_id: u16, transfer: u16, from_id: u16,
        to_id: u16, amount: f32) -> bool {
        todo!();
    }
}