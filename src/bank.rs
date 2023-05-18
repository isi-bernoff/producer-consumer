use std::thread::{JoinHandle, spawn};
use std::collections::{BTreeMap};
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fmt::{Display, Formatter, Result};
use self::transaction::{Mode, Transaction};
mod transaction;

/// #### Fields:
/// - `accounts`: `BTreeMap` of `u16` identifiers to `Mutex`-locked `f32` balances
/// - `ledger`: A `Vec` of `Transactions`
/// - `num_transactions`: The `u16` number of `Transaction`s in the `ledger`
/// - `num_succeeded`: The `u16` number of `Transaction`s that succeeded
#[derive(Debug)]
pub struct Bank {
    accounts: BTreeMap<u16, Mutex<f32>>,
    ledger: Vec<Transaction>,
    num_transactions: u16,
    num_succeeded: u16
}

impl Bank {
    /// Constructs a new `Bank` object and initializes its `accounts`, `ledger`, `num_transactions`,
    /// and `num_succeeded`
    /// #### Parameters
    /// - `num_accounts`: The number of `account`s to initialize
    /// - `ledger_filepath`: The name of a ledger file containing transactions formatted `<from_id>
    ///   <to_id> <amount> <mode_num>` on each line
    /// #### Returns
    /// The new `Bank` object
    pub fn new(num_accounts: u16, ledger_filepath: String) -> Self {
        let mut accounts: BTreeMap<u16, Mutex<f32>> = BTreeMap::new();
        let mut ledger: Vec<Transaction> = Vec::new();

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
            
            ledger.push(Transaction::new(id, from_id, to_id, amount, mode_id));
        }

        let num_transactions: u16 = u16::try_from(ledger.len()).unwrap();
        let num_succeeded: u16 = 0;

        Self {accounts, ledger, num_transactions, num_succeeded}
    }
    
    /// Spawns `Threads` to process this `Bank`'s `ledger` concurrently
    /// #### Parameters
    /// - `num_threads`: The number of `Thread`s to spawn
    pub fn start(self, num_threads: u16) {
        println!("{}", &self);
        let arc_bank = Arc::new(Mutex::new(self));
        let mut join_handles: Vec<JoinHandle<()>> = Vec::new();

        println!("Threads");

        for id in 0u16..num_threads {
            let arc_bank = Arc::clone(&arc_bank);
            let join_handle: JoinHandle<()> = spawn(move || {
                let thread_id: u16 = id;
                Bank::process_transaction(arc_bank, thread_id)
            });

            join_handles.push(join_handle);
        }

        for join_handle in join_handles {
            join_handle.join().unwrap();
        }

        println!("{}", *arc_bank.lock().unwrap());
    }

    /// Pops a `Transaction` from the `Bank`'s ledger and uses a `Thread`
    /// to process it concurrently
    /// #### Parameters
    /// - `arc_bank`: The atomic reference counter to this `Mutex`-locked `Bank`
    /// - `thread_id`: The identifier of the thread processing the `Transaction`
    pub fn process_transaction(arc_bank: Arc<Mutex<Bank>>, thread_id: u16) {
        let mut bank = arc_bank.lock().unwrap();

        while !(*bank).ledger.is_empty() {
            let transaction: Transaction = bank.ledger.pop().unwrap();
            let success_message: String = format!(" - Thread {:05} successfully processed {}",
                                                  thread_id, transaction);
            let failure_message: String = format!(" - Thread {:05} failed to process {}",
                                                  thread_id, transaction);

            match transaction.mode {
                Mode::Deposit => {
                    if !bank.deposit(transaction.to_id, transaction.amount) {
                        println!("{}", failure_message);
                    }

                    println!("{}", success_message);
                }
                Mode::Withdrawal => {
                    if !bank.withdraw(transaction.from_id, transaction.amount) {
                        println!("{}", failure_message);
                    }

                    println!("{}", success_message);
                }
                Mode::Transfer => {
                    if !bank.transfer(transaction.from_id, transaction.to_id, transaction.amount) {
                        println!("{}", failure_message);
                    }

                    println!("{}", success_message);
                }
            }
        }
    }
    
    /// Deposit money into one of this `Bank`'s `accounts`, incrementing `num_succeeded` if the
    /// deposit worked
    /// #### Parameters
    /// - `account_id`: The identifier of the account receiving the deposit
    /// - `amount`: The amount of money being deposited
    /// #### Returns
    /// `true` if the deposit succeeds and `false` otherwise
    pub fn deposit(&mut self, account_id: u16, amount: f32) -> bool {
        *self.accounts.get_mut(&account_id).unwrap().lock().unwrap() += amount;
        self.num_succeeded += 1;
        return true;
    }

    /// Withdraws money from one of this `Bank`'s `accounts`, incrementing `num_succeeded` if the
    /// withdrawal worked
    /// #### Parameters
    /// - `account_id`: The identifier of the account having its money withdrawn
    /// - `amount`: The amount of money being withdrawn
    /// #### Returns
    /// `true` if the withdrawal succeeds and `false` otherwise
    pub fn withdraw(&mut self, account_id: u16, amount: f32) -> bool {
        if amount > *self.accounts.get_mut(&account_id).unwrap().lock().unwrap() {
            return false;
        }

        *self.accounts.get_mut(&account_id).unwrap().lock().unwrap() -= amount;
        self.num_succeeded += 1;
        return true;
    }

    /// Transfers money from one of this `Bank`'s `accounts` to another, incrementing
    /// `num_succeeded` if the transfer worked
    /// #### Parameters
    /// - `from_id`: The identifier of the account having money transferred from it
    /// - `to_id`: The identifier of the account having money transferred to it
    /// - `amount`: The amount of money being transferred
    /// #### Returns
    /// `true` if the transfer succeeds and `false` otherwise
    pub fn transfer(&mut self, from_id: u16, to_id: u16, amount: f32) -> bool {
        if from_id == to_id {
            return false;
        }

        if amount > *self.accounts.get_mut(&from_id).unwrap().lock().unwrap() {
            self.num_succeeded += 1;
            return false;
        }

        *self.accounts.get_mut(&from_id).unwrap().lock().unwrap() -= amount;
        *self.accounts.get_mut(&to_id).unwrap().lock().unwrap() += amount;
        self.num_succeeded += 1;
        return true;
    }
}

impl Display for Bank {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter, "Bank\n")?;

        for (id, balance) in &self.accounts {
            let id: u16 = *id;
            let balance: f32 = *balance.lock().unwrap();

            write!(formatter, " - Account {:05}: ${:.2}\n", id, balance)?;
        }

        write!(formatter, "Ledger: {}/{} transactions succeeded\n",
               self.num_succeeded, self.num_transactions)?;

        for transaction in &self.ledger {
            write!(formatter, " - {}\n", transaction)?;
        }
        Ok(())
    }
}