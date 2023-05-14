#![crate_name = "bank"]

use std::env::args;
use bank::Bank;
mod bank;

fn main() {
    let args: Vec<String> = args().collect();
    let num_accounts: u16 = 10;
    let ledger_filepath: String = args[2].parse::<String>().unwrap();
    let bank: Bank = Bank::new(num_accounts, ledger_filepath);
    let num_threads: u16 = args[1].parse::<u16>().unwrap();
    bank.init(num_threads);
}