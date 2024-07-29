use std::time::SystemTime;

#[derive(Debug, PartialEq)]
pub enum TransectionType {
    Credit,
    Debit,
}

#[derive(Debug)]
pub struct Transection {
    _party: String,
    pub amount: i32,
    pub transection_type: TransectionType,
    pub timestamp: SystemTime,
}

impl Transection {
    pub fn new(party: &str, amount: i32, transection_type: TransectionType) -> Self {
        Self {
            _party: party.to_string(),
            amount,
            transection_type,
            timestamp: SystemTime::now(),
        }
    }
}
