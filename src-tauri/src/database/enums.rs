use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use diesel::{AsExpression, FromSqlRow};

#[derive(AsExpression, FromSqlRow, Debug, Clone, Copy, PartialEq, Eq)]
#[diesel(sql_type = Text)]
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

impl ToSql<Text, Sqlite> for LimitInterval {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let text = match self {
            LimitInterval::Hourly => "hourly",
            LimitInterval::Daily => "daily",
            LimitInterval::Weekly => "weekly",
            LimitInterval::Biweekly => "biweekly",
            LimitInterval::Monthly => "monthly",
            LimitInterval::Quarterly => "quarterly",
            LimitInterval::Semiannually => "semiannually",
            LimitInterval::Annually => "annually",
            LimitInterval::Biennially => "biennially",
            LimitInterval::Custom => "custom",
        };

        <str as ToSql<Text, Sqlite>>::to_sql(text, out)
    }
}

impl FromSql<Text, Sqlite> for LimitInterval {
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let text: String = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
        match text.as_str() {
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(AsExpression, FromSqlRow, Debug, Clone, Copy, PartialEq, Eq)]
#[diesel(sql_type = Text)]
pub enum TransactionStatus {
    Completed,
    Declined,
    Pending,
}

impl ToSql<Text, Sqlite> for TransactionStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let text = match self {
            TransactionStatus::Completed => "completed",
            TransactionStatus::Declined => "declined",
            TransactionStatus::Pending => "pending",
        };

        <str as ToSql<Text, Sqlite>>::to_sql(text, out)
    }
}

impl FromSql<Text, Sqlite> for TransactionStatus {
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let text: String = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
        match text.as_str() {
            "completed" => Ok(TransactionStatus::Completed),
            "declined" => Ok(TransactionStatus::Declined),
            "pending" => Ok(TransactionStatus::Pending),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(AsExpression, FromSqlRow, Debug, Clone, Copy, PartialEq, Eq)]
#[diesel(sql_type = Text)]
pub enum TransactionCategory {
    Inflow,
    Outflow,
}

impl ToSql<Text, Sqlite> for TransactionCategory {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let text = match self {
            TransactionCategory::Inflow => "inflow",
            TransactionCategory::Outflow => "outflow",
        };

        <str as ToSql<Text, Sqlite>>::to_sql(text, out)
    }
}

impl FromSql<Text, Sqlite> for TransactionCategory {
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let text: String = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
        match text.as_str() {
            "inflow" => Ok(TransactionCategory::Inflow),
            "outflow" => Ok(TransactionCategory::Outflow),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(AsExpression, FromSqlRow, Debug, Clone, Copy, PartialEq, Eq)]
#[diesel(sql_type = Text)]
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

impl ToSql<Text, Sqlite> for TransactionMedium {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let text = match self {
            TransactionMedium::Cash => "cash",
            TransactionMedium::BankTransfer => "bank transfer",
            TransactionMedium::CreditCard => "credit card",
            TransactionMedium::DebitCard => "debit card",
            TransactionMedium::DigitalPayment => "digital payment",
            TransactionMedium::Crypto => "crypto",
            TransactionMedium::Check => "check",
            TransactionMedium::MobileWallet => "mobile wallet",
            TransactionMedium::Other => "other",
        };

        <str as ToSql<Text, Sqlite>>::to_sql(text, out)
    }
}

impl FromSql<Text, Sqlite> for TransactionMedium {
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let text: String = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
        match text.as_str() {
            "cash" => Ok(TransactionMedium::Cash),
            "bank transfer" => Ok(TransactionMedium::BankTransfer),
            "credit card" => Ok(TransactionMedium::CreditCard),
            "debit card" => Ok(TransactionMedium::DebitCard),
            "digital payment" => Ok(TransactionMedium::DigitalPayment),
            "crypto" => Ok(TransactionMedium::Crypto),
            "check" => Ok(TransactionMedium::Check),
            "mobile wallet" => Ok(TransactionMedium::MobileWallet),
            "other" => Ok(TransactionMedium::Other),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
