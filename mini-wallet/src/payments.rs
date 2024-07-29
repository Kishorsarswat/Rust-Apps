use std::{fmt, str::FromStr};
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Eq, Clone, EnumIter)]
pub enum PaymentOptions {
    CC,
    DC,
    UPI,
}

impl FromStr for PaymentOptions {
    type Err = ();

    fn from_str(input: &str) -> Result<PaymentOptions, Self::Err> {
        match input {
            "CC" => Ok(PaymentOptions::CC),
            "DC" => Ok(PaymentOptions::DC),
            "UPI" => Ok(PaymentOptions::UPI),
            _ => Err(()),
        }
    }
}

impl fmt::Display for PaymentOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PaymentOptions::CC => write!(f, "CC"),
            PaymentOptions::DC => write!(f, "DC"),
            PaymentOptions::UPI => write!(f, "UPI"),
        }
    }
}
