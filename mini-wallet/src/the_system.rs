use crate::payments::PaymentOptions;
use crate::transection::Transection;
use crate::user::User;
use std::collections::HashMap;
use std::io::ErrorKind;
use std::str::FromStr;

#[derive(Default)]
pub struct TheSystem {
    users: HashMap<String, User>,
}

impl TheSystem {
    pub fn register_user(&mut self, user_name: &str) -> Result<(), ErrorKind> {
        if self.users.contains_key(user_name) {
            return Err(ErrorKind::AlreadyExists);
        }
        self.users
            .insert(user_name.to_string(), User::new(user_name));
        Ok(())
    }

    pub fn fetch_balance(&self, user_name: &str) -> Result<i32, ErrorKind> {
        if let Some(user) = self.users.get(user_name) {
            Ok(user.fetch_balance())
        } else {
            Err(ErrorKind::NotFound)
        }
    }

    pub fn top_up(&mut self, user_name: &str, source: &str, amount: i32) -> Result<(), ErrorKind> {
        if let Some(user) = self.users.get_mut(user_name) {
            if amount <= 0 {
                return Err(ErrorKind::InvalidData);
            }
            if !is_valid_payment_option(source) {
                return Err(ErrorKind::InvalidData);
            }
            user.top_up(source, amount)?;
            Ok(())
        } else {
            Err(ErrorKind::NotFound)
        }
    }

    pub fn create_deposite(
        &mut self,
        user_name: &str,
        account: &str,
        amount: i32,
    ) -> Result<(), ErrorKind> {
        if amount <= 0 {
            return Err(ErrorKind::InvalidData);
        }
        if let Some(user) = self.users.get_mut(user_name) {
            user.create_deposite(account, amount)
                .map_err(|_| ErrorKind::Other)
        } else {
            Err(ErrorKind::NotFound)
        }
    }

    pub fn book_deposite(&mut self, user_name: &str, account: &str) -> Result<(), ErrorKind> {
        if let Some(user) = self.users.get_mut(user_name) {
            user.book_deposite(account).map_err(|_| ErrorKind::Other)
        } else {
            Err(ErrorKind::NotFound)
        }
    }

    pub fn send_money(
        &mut self,
        src_user_name: &str,
        dest_user_name: &str,
        amount: i32,
    ) -> Result<(), ErrorKind> {
        let src_user = self
            .users
            .get_mut(src_user_name)
            .ok_or(ErrorKind::NotFound)?;
        src_user
            .send_money(dest_user_name, amount)
            .map_err(|_| ErrorKind::Other)?;
        let dest_user = self
            .users
            .get_mut(dest_user_name)
            .ok_or(ErrorKind::NotFound)?;
        dest_user.receive_money(src_user_name, amount);
        Ok(())
    }

    pub fn get_transections(
        &self,
        user_name: &str,
        credit_or_debit: &str,
        sort_by: &str,
    ) -> Result<Vec<&Transection>, ErrorKind> {
        if let Some(user) = self.users.get(user_name) {
            Ok(user.get_transections(credit_or_debit, sort_by))
        } else {
            Err(ErrorKind::NotFound)
        }
    }
}

pub fn is_valid_payment_option(option_str: &str) -> bool {
    PaymentOptions::from_str(option_str).is_ok()
}
