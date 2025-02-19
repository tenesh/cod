#[derive(Debug)]
pub enum LimitInterval {
    Hourly,
    Daily,
    Weekly,
    Biweekly,
    Monthly,
    Quarterly,
    Semiannually,
    Annually,
    Biennially,
    Custom,
}

#[derive(Debug)]
pub enum TransactionStatus {
    Completed,
    Declined,
    Pending,
}

#[derive(Debug)]
pub enum TransactionCategory {
    Inflow,
    Outflow,
}

#[derive(Debug)]
pub enum TransactionMedium {
    Cash,
    BankTransfer,
    CreditCard,
    DebitCard,
    DigitalPayment,
    Crypto,
    Check,
    MobileWallet,
    Other,
}
