/// Initializes a new `Bank` with 10 `Account`s, creates `num_workers`
/// `Thread`s, and loads ledger file `filename` with `load_ledger`
/// ### Arguments
/// * `num_workers` - The number of worker `Thread`s
/// * `filename` - The filename of a ledger file
pub fn init_bank(num_threads: u16, filename: String) {}

/// Parses ledger file `filename` containing transations formatted
/// `<from account_id> <to_account_id> <amount> <mode>` on each line into the
/// global `ledger`
/// ### Arguments
/// * `filename` - The filename of a ledger file
fn load_ledger(filename: String) {}

/// Pops a `Transaction` from the global `ledger` and executes it concurrently
/// with the worker `Thread` identified `worker_id`
/// ### Arguments
/// * `worker_id` - The identifier of a worker `Thread`
fn worker(worker_id: u16) {}