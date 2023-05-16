use std::collections::{BTreeMap};
use std::thread::{JoinHandle};
use std::sync::Mutex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fmt::{Display, Formatter, Result};
use self::transaction::Transaction;
mod transaction;

/// #### Fields:
/// - `join_handles`: A `BTreeMap` of `u16` identifiers to `String` messages returned from
///   `Threads`s' `JoinHandle`s
/// - `accounts`: A `BTreeMap` of `u16` identifiers to `Mutex`-locked `f32` balances
/// - `ledger`: A `BTreeMap` of `Transactions`
/// - `num_successful`: The `u16` number of `Transaction`s that succeeded
/// - `num_failed`: The `u16` number of `Transaction`s that failed
pub struct Bank {
    join_handles: BTreeMap<u16, JoinHandle<String>>,
    accounts: BTreeMap<u16, Mutex<f32>>,
    ledger: BTreeMap<u16, Transaction>,
    num_successful: u16,
    num_failed: u16
}

impl Bank {
    /// Constructs a new `Bank` object and initializes its `accounts`, `ledger`, `num_successful`,
    /// and `num_failed`
    /// #### Parameters
    /// - `num_accounts`: The number of `account`s to initialize
    /// - `ledger_filepath`: The name of a ledger file containing transactions formatted `<from_id>
    ///   <to_id> <amount> <mode_num>` on each line
    /// #### Returns
    /// The new `Bank` object
    pub fn new(num_accounts: u16, ledger_filepath: String) -> Self {
        let join_handles: BTreeMap<u16, JoinHandle<String>> = BTreeMap::new();
        let mut accounts: BTreeMap<u16, Mutex<f32>> = BTreeMap::new();
        let mut ledger: BTreeMap<u16, Transaction> = BTreeMap::new();
        let num_successful: u16 = 0;
        let num_failed: u16 = 0;

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

        Self {join_handles, accounts, ledger, num_successful, num_failed}
    }
    
    /// Spawns `Threads` to initialize this `Bank`'s `join_handles`
    /// #### Parameters
    /// - `num_threads`: The number of `Thread`s to spawn
    pub fn spawn(&mut self, num_threads: u16) {
        todo!();
    }

    /// Pops a `Transaction` from the `Bank`'s ledger and uses a `Thread`
    /// to process it concurrently
    /// #### Returns
    /// A success message if the `Transaction` succeeds and a failure message otherwise
    pub fn perform_transaction(&mut self) -> String {
        todo!();
    }
    
    /// Deposit money into one of this `Bank`'s `accounts`, incrementing
    /// `num_successful` or `num_failed` depending on if the deposit works
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
    /// `num_successful` or `num_failed` depending on if the withdrawal works
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
    /// incrementing `num_successful` or `num_failed` depending on if the
    /// transfer works
    /// #### Parameters
    /// - `thread_id`: The identifier of the `Thread` processing the the transfer
    /// - `transaction_id`: The identifier of the transfer `Transaction`
    /// - `from_id`: The identifier of the account having money transferred from it
    /// - `to_id`: The identifier of the account having money transferred to it
    /// - `amount`: The amount of money being transferred
    /// #### Returns
    /// `true` if the transfer succeeds and `false` otherwise
    pub fn transfer(&self, thread_id: u16, transfer: u16, from_id: u16,
        to_id: u16, amount: f32) -> bool {
        todo!();
    }
}

impl Display for Bank {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter, "Bank\n")?;

        for (id, mutex) in &self.accounts {
            let id: u16 = *id;
            let balance: f32 = *mutex.lock().unwrap();

            write!(formatter, " - Account {:05}: ${:.2}\n", id, balance)?;
        }

        write!(formatter, "Transactions\n - Successful: {}\n - Failed: {}",
               self.num_successful, self.num_failed)
    }
}