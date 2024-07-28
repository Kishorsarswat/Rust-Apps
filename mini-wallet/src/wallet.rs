#[derive(Default)]
pub struct Wallet {
    balance: i32,
}

impl Wallet {
    pub fn get_balance(&self) -> i32 {
        self.balance
    }

    pub fn add_balance(&mut self, amount: i32) {
        self.balance += amount;
    }

    pub fn subtract_balance(&mut self, amount: i32) {
        self.balance -= amount;
    }
}
