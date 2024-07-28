use std::{collections::HashMap, io::ErrorKind};

use strum::IntoEnumIterator;

use crate::{
    payments::PaymentOptions,
    transection::{Transection, TransectionType},
    wallet::Wallet,
};

const MAX_AMOUNT_ALLOWED_VIA_UPI: i32 = 1000;
const THERSHOLD_DC_TRANSECTION: u32 = 3;
const MAX_CREDIT_CARD_TRANSECTIONS_ALLOWED: u32 = 3;
const DEDUCTABLE_AMOUNT: i32 = 1;

pub struct User {
    _name: String,
    wallet: Wallet,
    deposite: HashMap<String, i32>,
    transections: Vec<Transection>,
    transection_count: HashMap<String, u32>,
}

impl User {
    pub fn new(name: &str) -> Self {
        let mut transection_count = HashMap::new();
        for option in PaymentOptions::iter() {
            let option = option.to_string();
            transection_count.insert(option, 0);
        }
        Self {
            _name: name.to_string(),
            wallet: Wallet::default(),
            deposite: HashMap::new(),
            transections: Vec::new(),
            transection_count,
        }
    }

    pub fn fetch_balance(&self) -> i32 {
        self.wallet.get_balance()
    }

    pub fn top_up(&mut self, src: &str, amount: i32) -> Result<(), ErrorKind> {
        match self.check_business_logic(src, amount) {
            Ok(x) => self.wallet.add_balance(amount - x),
            Err(_e) => return Err(ErrorKind::Other),
        }
        self.transections
            .push(Transection::new(src, amount, TransectionType::Credit));
        let count = self.transection_count.get_mut(src).unwrap();
        *count += 1;
        Ok(())
    }

    pub fn create_deposite(&mut self, account: &str, amount: i32) -> Result<(), ErrorKind> {
        if amount > self.wallet.get_balance() {
            return Err(ErrorKind::InvalidData);
        }
        if let Some(balance) = self.deposite.get_mut(account) {
            *balance += amount;
        } else {
            self.deposite.insert(account.to_string(), amount);
        }
        self.wallet.subtract_balance(amount);
        self.transections
            .push(Transection::new(account, amount, TransectionType::Debit));
        Ok(())
    }

    pub fn book_deposite(&mut self, account: &str) -> Result<(), ErrorKind> {
        if let Some(amount) = self.deposite.remove(account) {
            self.wallet.add_balance(amount);
            self.transections
                .push(Transection::new(account, amount, TransectionType::Credit));
        } else {
            return Err(ErrorKind::InvalidData);
        }
        Ok(())
    }

    pub fn send_money(&mut self, dest: &str, amount: i32) -> Result<(), ErrorKind> {
        if self.wallet.get_balance() < amount {
            return Err(ErrorKind::InvalidData);
        }
        self.wallet.subtract_balance(amount);
        self.transections
            .push(Transection::new(dest, amount, TransectionType::Debit));
        Ok(())
    }

    pub fn receive_money(&mut self, src: &str, amount: i32) {
        self.wallet.add_balance(amount);
        self.transections
            .push(Transection::new(src, amount, TransectionType::Credit));
    }

    pub fn get_transections(&self, credit_or_debit: &str, sort_by: &str) -> Vec<&Transection> {
        let mut filtered_transections: Vec<&Transection> = self
            .transections
            .iter()
            .filter(|t| match credit_or_debit.to_lowercase().as_str() {
                "credit" => t.transection_type == TransectionType::Credit,
                "debit" => t.transection_type == TransectionType::Debit,
                _ => true,
            })
            .collect();

        match sort_by.to_lowercase().as_str() {
            "time" => filtered_transections.sort_by_key(|t| t.timestamp),
            "amount" => filtered_transections.sort_by_key(|t| t.amount),
            _ => (),
        }
        filtered_transections
    }

    fn check_business_logic(&self, src: &str, amount: i32) -> Result<i32, ErrorKind> {
        if src == "CC"
            && self.transection_count.get(src).unwrap() >= &MAX_CREDIT_CARD_TRANSECTIONS_ALLOWED
        {
            return Err(ErrorKind::Other);
        }
        if src == "UPI" && amount > MAX_AMOUNT_ALLOWED_VIA_UPI {
            return Err(ErrorKind::Other);
        }
        if src == "DC" && self.transection_count.get(src).unwrap() >= &THERSHOLD_DC_TRANSECTION {
            return Ok(DEDUCTABLE_AMOUNT);
        }
        Ok(0)
    }
}
