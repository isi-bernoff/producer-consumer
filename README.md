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
- `threads`: A `HashMap` of `u16` identifiers to `Threads`
- `accounts`: A `HashMap` of `u16` identifiers to `Mutex`-locked `f32` balances
- `ledger`: A `HashMap` of `u16` identifiers to `Transactions`
- `num_successes`: The `u16` number of `Transaction`s that succeeded
- `num_failures`: The `u16` number of `Transaction`s that failed
## Methods
```rs
Transaction::new(from_id: u16, to_id: u16, amount: f32, mode_id: u8) -> Transaction
```
Constructs and initializes a new `Transaction` object
#### Parameters
- `from_id`: The identifier of the account having its money removed
- `to_id`: The identifier of the account receiving money
- `amount`: The amount of money being moved
- `mode_id`: The identifier for the mode of the `Transaction`, where `0`, `1`, and `2` represent a deposit, withdrawal, or transfer, respectively.
#### Returns
The new `Transaction` object
```rs
Bank::new(num_accounts: u16, ledger_filepath: String) -> Bank
```
Constructs a new `Bank` object and initializes its accounts, ledger, and numbers of successful and failed transactions
#### Parameters
- `num_accounts`: The number of `account`s to initialize
- `ledger_filepath`: The name of a ledger file containing transactions formatted `<from_id> <to_id> <amount> <mode_id>` on each line
#### Returns
The new `Bank` object
```rs
Bank::init(num_threads: u16)
```
Initializes this `Bank`'s `threads` and `ledger`
#### Parameters
- `num_threads`: The number of `Thread`s to initialize
```rs
Bank::deposit(thread_id: u16, transaction_id: u16, account_id: u16, amount: f32) -> String
```
Deposit money into one of this `Bank`'s `accounts`, incrementing
`num_successes` or `num_failures` depending on if the deposit works
#### Parameters
- `thread_id`: The identifier of the `Thread` processing the the deposit
- `transaction_id`: The identifier of the deposit `Transaction`
- `account_id`: The identifier of the account receiving the deposit
- `amount`: The amount of money being deposited
#### Returns
A success message if the deposit succeeds and a failure message
otherwise
```rs
Bank::withdraw(thread_id: u16, transaction_id: u16, account_id: u16, amount: f32) -> String
```
Withdraws money from one of this `Bank`'s `accounts`, incrementing `num_successes` or `num_failures` depending on if the withdrawal works
#### Parameters
- `thread_id`: The identifier of the `Thread` processing the the withdrawal
- `transaction_id`: The identifier of the withdrawal `Transaction`
- `account_id`: The identifier of the account having the withdrawal
- `amount`: The amount of money being withdrawn
#### Returns
A success message if the withdrawal succeeds and a failure message
otherwise
```rs
Bank::transfer(thread_id: u16, transfer: u16, from_id: u16, to_id: u16, amount: f32) -> String
```
Transfers money from one of this `Bank`'s `accounts` to another,
incrementing `num_successes` or `num_failures` depending on if the
transfer works
#### Parameters
- `thread_id`: The identifier of the `Thread` processing the the transfer
- `transaction_id`: The identifier of the transfer `Transaction`
- `from_id`: The identifier of the account having money transferred from it
- `to_id`: The identifier of the account having money transferred to it
- `amount`: The amount of money being withdrawn
#### Returns
A success message if the transfer succeeds and a failure message otherwise
```rs
Bank::thread(id: u16)
```
Pops a `Transaction` from the `Bank`'s `ledger` and uses a `Thread` to process it concurrently
#### Parameters
- `id`: The identifier of the `Thread` processing the `Transaction`