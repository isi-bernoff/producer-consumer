use std::sync::Mutex;

struct Account {
    id: u32, 
    balance: Mutex<i32>,
}

struct Bank {
    n: u16,
    successes: u16,
    failures: u16
}