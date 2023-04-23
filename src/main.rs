#![crate_name = "cs_377_final_project"]

mod ledger;
use std::env::args;
use std::process::exit;
use crate::ledger::init_bank;

fn main() {
    let args: Vec<String> = args().collect();
    
    if args.len() != 3 {
        eprintln!("Usage: {} <num_of_threads> <leader_file>", args[0]);
        exit(-1);
    }

    let n: u8 = args[1].parse().unwrap();
    init_bank(n, args[2].clone());
}