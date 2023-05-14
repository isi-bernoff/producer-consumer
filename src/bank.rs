use std::collections::HashMap;
use std::thread::Thread;
use std::sync::Mutex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fmt::{Formatter, Display, Result};
use self::transaction::Transaction;
mod transaction;

/// #### Fields:
/// - `threads`: A `HashMap` of `u16` identifiers to `Threads`
/// - `accounts`: A `HashMap` of `u16` identifiers to `Mutex`-locked `i32`
///    balances
/// - `ledger`: A `HashMap` of `u16` identifiers to `Transactions`
/// - `num_successes`: The `u16` number of `Transaction`s that succeeded
/// - `num_failures`: The `u16` number of `Transaction`s that failed
pub struct Bank {
    threads: HashMap<u16, Thread>,
    accounts: HashMap<u16, Mutex<i32>>,
    ledger: HashMap<u16, Transaction>,
    num_successes: u16,
    num_failures: u16
}

impl Bank {
    /// Constructs a new `Bank` object and initializes its `accounts` and
    /// `ledger`
    /// #### Parameters
    /// - `num_accounts`: The number of `account`s to initialize
    /// - `ledger_filepath`: The name of a ledger file containing transactions
    /// formatted `<from_id> <to_id> <amount> <mode_num>` on each line
    /// #### Returns
    /// The new `Bank` object
    pub fn new(num_accounts: u16, ledger_filepath: String) -> Self {
        let threads: HashMap<u16, Thread> = HashMap::new();
        let mut accounts: HashMap<u16, Mutex<i32>> = HashMap::new();
        let mut ledger: HashMap<u16, Transaction> = HashMap::new();
        let num_successes: u16 = 0;
        let num_failures: u16 = 0;

        for id in 0..num_accounts {
            accounts.insert(id, Mutex::new(0));
        }

        let file: File = File::open(ledger_filepath).unwrap();
        let reader: BufReader<File> = BufReader::new(file);
        
        for (id, line) in reader.lines().enumerate() {
            let id: u16 = u16::try_from(id).unwrap();
            let line: String = line.unwrap();
            let mut tokens = line.splitn(4, ' ');
            let from_id: u16 = tokens.next().unwrap().parse::<u16>().unwrap();
            let to_id: u16 = tokens.next().unwrap().parse::<u16>().unwrap();
            let amount: i32 = tokens.next().unwrap().parse::<i32>().unwrap();
            let mode_id: u8 = tokens.next().unwrap().parse::<u8>().unwrap();
            let transaction: Transaction = Transaction::new(from_id, to_id, amount, mode_id);
            ledger.insert(id, transaction);
        }

        Self {threads, accounts, ledger, num_successes, num_failures}
    }
    
    /// Initializes this `Bank`'s `threads`
    /// #### Parameters
    /// - `num_threads`: The number of `Thread`s to initialize
    pub fn init(&self, num_threads: u16) {
        todo!();
    }
    
    /// Deposit money into one of this `Bank`'s `accounts`, incrementing
    /// `num_successes` or `num_failures` depending on if the deposit works
    /// #### Parameters
    /// - `thread_id`: The identifier of the `Thread` processing the the
    ///   deposit
    /// - `transaction_id`: The identifier of the deposit `Transaction`
    /// - `account_id`: The identifier of the account receiving the deposit
    /// - `amount`: The amount of money being deposited
    /// #### Returns
    /// A success message if the deposit succeeds and a failure message
    /// otherwise
    pub fn deposit(thread_id: u16, transaction_id: u16, account_id: u16,
                   amount: i32) -> String {
        todo!();
    }

    /// Withdraws money from one of this `Bank`'s `accounts`, incrementing
    /// `num_successes` or `num_failures` depending on if the withdrawal works
    /// #### Parameters
    /// - `thread_id`: The identifier of the `Thread` processing the the
    ///   withdrawal
    /// - `transaction_id`: The identifier of the withdrawal `Transaction`
    /// - `account_id`: The identifier of the account having the withdrawal
    /// - `amount`: The amount of money being withdrawn
    /// #### Returns
    /// A success message if the withdrawal succeeds and a failure message
    /// otherwise
    pub fn withdraw(thread_id: u16, transaction_id: u16, account_id: u16,
        amount: i32) -> String {
        todo!();
    }

    /// Transfers money from one of this `Bank`'s `accounts` to another,
    /// incrementing `num_successes` or `num_failures` depending on if the
    /// transfer works
    /// #### Parameters
    /// - `thread_id`: The identifier of the `Thread` processing the the
    ///   transfer
    /// - `transaction_id`: The identifier of the transfer `Transaction`
    /// - `from_id`: The identifier of the account having money transferred
    ///   from it
    /// - `to_id`: The identifier of the account having money transferred
    ///   to it
    /// - `amount`: The amount of money being withdrawn
    /// #### Returns
    /// A success message if the transfer succeeds and a failure message
    /// otherwise
    pub fn transfer(thread_id: u16, transfer: u16, from_id: u16,
        to_id: u16, amount: i32) -> String {
        todo!();
    }

    /// Pops a `Transaction` from the `Bank`'s `ledger` and uses a `Thread`
    /// to process it concurrently
    /// #### Parameters
    /// `id`: The identifier of the `Thread` processing the `Transaction`
    pub fn thread(id: u16) {
        todo!();
    }
}

impl Drop for Bank {
    fn drop(&mut self) {
        todo!();
    }
}

impl Display for Bank {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        todo!();
    }
}