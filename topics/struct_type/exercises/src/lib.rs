#[derive(Debug)]
pub struct Account {
    pub address: String,
    pub balance: u32,
}

pub fn new(address: String) -> Account {
    Account {address: address, balance: 0}
}
