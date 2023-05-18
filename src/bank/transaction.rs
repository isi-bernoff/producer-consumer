use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Mode {
    Deposit,
    Withdrawal,
    Transfer
}

/// #### Fields:
/// - `id`: The `u16` identifier for this `Transaction`
/// - `from_id`: The `u16` identifier of the account having its money removed
/// - `to_id`: The `u16` identifier of the account having receiving money
/// - `amount`: The `i32` amount of money being moved
/// - `mode`: The `enum Mode` of transaction, a `Deposit`, `Withdrawal`, or
///   `Transfer`
#[derive(Debug)]
pub struct Transaction {
    pub id: u16,
    pub from_id: u16,
    pub to_id: u16,
    pub amount: f32,
    pub mode: Mode
}

impl Transaction {
    /// Constructs and initializes a new `Transaction` object
    /// #### Parameters
    /// - `id`: The identifier for the new `Transaction` object
    /// - `from_id`: The identifier of the account having its money moved
    /// - `to_id`: The identifier of the account receiving money
    /// - `amount`: The amount of money being moved
    /// - `mode_id`: The identifier for the mode of the `Transaction`, where `0`, `1`, and `2`
    ///   represent a deposit, withdrawal, or transfer, respectively.
    /// #### Returns
    /// The new `Transaction` object
    pub fn new(id: u16, from_id: u16, to_id: u16, amount: f32, mode_id: u8) -> Self {
        let mode: Mode = match mode_id {
            0 => Mode::Deposit,
            1 => Mode::Withdrawal,
            2 => Mode::Transfer,
            _ => panic!("Invalid mode identifier")
        };

        Self {id, from_id, to_id, amount, mode}
    }
}

impl Display for Transaction {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self.mode {
            Mode::Deposit => write!(formatter,
                                    "Transaction {:05}: Deposit of ${:.2} into Account {:05}",
                                    self.id, self.amount, self.to_id),
            Mode::Withdrawal => write!(formatter,
                                    "Transaction {:05}: Withdrawal of ${:.2} from Account {:05}",
                                    self.id, self.amount, self.from_id),
            Mode::Transfer => write!(formatter,
                                    "Transaction {:05}: Transfer of ${:.2} from Account {:05} to Account {:05}",
                                    self.id, self.amount, self.from_id, self.to_id)
        }
    }
}