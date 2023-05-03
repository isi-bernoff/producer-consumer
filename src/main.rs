#![crate_name = "multi_bank"]

use std::env::args;
use std::process::exit;
use crate::bank::Bank;
mod bank;

fn main() {
    let args: Vec<String> = args().collect();
    
    if args.len() != 3 {
        eprintln!("Usage: {} <num_threads> <ledger_filename>", args[0]);
        exit(-1);
    }

    let num_threads: u16 = args[1].parse().unwrap();
    let ledger_filename: String = args[2].clone();
    let bank: Bank = Bank::new(10, ledger_filename);
    bank.init(num_threads);
}