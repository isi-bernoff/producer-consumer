# Bank
This project is a translation of [UMass](https://www.umass.edu)'s [Operating Systems](https://www.cics.umass.edu/content/spring-23-course-descriptions#377) TAs [Dave Dirnfield](https://github.com/dd2912) and [Calvin Chai](https://github.com/NightDawnEX)'s C/C++ [Producer/Consumer](https://github.com/umass-cs-377/umass-cs-377.github.io/blob/a18ded192a18bc59affecb8ef077849617b3a61d/docs/04-projects/04-prodcon/index.md) project into [Rust](https://www.https://www.rust-lang.org). It is a banking application that uses threads to process transactions from a ledger file concurrently.
## Prerequisites
Rust 1.65.0+
## Compilation
```console
$ cargo build
```
## Execution
```console
$ cargo run <num_threads> <ledger_filename>
```
## Structures
### `Transaction`
#### Fields
- `from_id`: The `u16` identifier of the account having its money removed
- `to_id`: The `u16` identifier of the account having receiving money
- `amount`: The `f32` amount of money being moved
- `mode`: The `enum Mode` of transaction, a `Deposit`, `Withdrawal`, or `Transfer`
### `Bank`
#### Fields
- `join_handles`: A `BTreeMap` of `u16` identifiers to `Threads`s' `JoinHandle`s
- `accounts`: A `BTreeMap` of `u16` identifiers to `Mutex`-locked `f32` balances
- `ledger`: A `BTreeMap` of `u16` identifiers to `Transactions`
- `num_successful`: The `u16` number of `Transaction`s that succeeded
- `num_failed`: The `u16` number of `Transaction`s that failed
## Methods
```rs
Transaction::new(from_id: u16, to_id: u16, amount: f32, mode_id: u8) -> Transaction
```
Constructs and initializes a new `Transaction` object
#### Parameters
- `from_id`: The identifier of the account having its money removed
- `to_id`: The identifier of the account receiving money
- `amount`: The amount of money being moved
- `mode_id`: The identifier for the mode of the `Transaction`, where `0`, `1`, and `2` represent a deposit, withdrawal, or transfer, respectively
#### Returns
The new `Transaction` object
```rs
Bank::new(num_accounts: u16, ledger_filepath: String) -> Bank
```
Constructs a new `Bank` object and initializes its `accounts`, `ledger`, `num_successful`, and `num_failed`
#### Parameters
- `num_accounts`: The number of `account`s to initialize
- `ledger_filepath`: The name of a ledger file containing transactions formatted `<from_id> <to_id> <amount> <mode_id>` on each line
#### Returns
The new `Bank` object
```rs
Bank::spawn(&mut self, num_threads: u16)
```
Spawns `Threads` to initialize this `Bank`'s `join_handles`
#### Parameters
- `num_threads`: The number of `Thread`s to spawn
```rs
pub fn perform_transaction(&mut self, thread_id: u16) -> String
```
Pops a `Transaction` from the `Bank`'s ledger and uses a `Thread` to process it concurrently
#### Parameters
`thread_id`: The identifier of the thread processing the `Transaction`
#### Returns
A success message if the `Transaction` succeeds and a failure message otherwise
    
```rs
Bank::deposit(&mut self, thread_id: u16, transaction_id: u16, account_id: u16, amount: f32) -> bool
```
Deposit money into one of this `Bank`'s `accounts`, incrementing
`num_successful` or `num_failed` depending on if the deposit works
#### Parameters
- `thread_id`: The identifier of the `Thread` processing the the deposit
- `transaction_id`: The identifier of the deposit `Transaction`
- `account_id`: The identifier of the account receiving the deposit
- `amount`: The amount of money being deposited
#### Returns
`true` if the deposit succeeds and `false` otherwise
```rs
Bank::withdraw(&mut self, thread_id: u16, transaction_id: u16, account_id: u16, amount: f32) -> bool
```
Withdraws money from one of this `Bank`'s `accounts`, incrementing `num_successful` or `num_failed` depending on if the withdrawal works
#### Parameters
- `thread_id`: The identifier of the `Thread` processing the the withdrawal
- `transaction_id`: The identifier of the withdrawal `Transaction`
- `account_id`: The identifier of the account having the withdrawal
- `amount`: The amount of money being withdrawn
#### Returns
`true` if the withdrawal succeeds and `false` otherwise
```rs
Bank::transfer(&mut self, thread_id: u16, transfer: u16, from_id: u16, to_id: u16, amount: f32) -> bool
```
Transfers money from one of this `Bank`'s `accounts` to another,
incrementing `num_successful` or `num_failed` depending on if the
transfer works
#### Parameters
- `thread_id`: The identifier of the `Thread` processing the the transfer
- `transaction_id`: The identifier of the transfer `Transaction`
- `from_id`: The identifier of the account having money transferred from it
- `to_id`: The identifier of the account having money transferred to it
- `amount`: The amount of money being transferred
#### Returns
`true` if the transfer succeeds and `false` otherwise