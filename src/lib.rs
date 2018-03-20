extern crate chrono;
use chrono::NaiveDate;

use std::fs;
use std::io;

#[derive(Debug)]
struct Amount(i64);

/// An account is a real world banking product where money transactions occur.
/// These transactions will be categorized in to envelopes to be budgeted.
#[derive(Debug)]
pub struct Account {
    name: String,
    kind: AccountKind,
}

#[derive(Debug)]
enum AccountKind {
    Savings,
    Checking,
    CreditCard,
    Loan,
    Investment,
}

/// An Envelope is a virtual account backed by actual funds in the Accounts . Funds are transferred
/// between Envelopes, often from an Income Envelopes to Expense Envelopes . Transactions are made
/// against Envelopes . This allows each Envelope to have a balance (positive or negative).
#[derive(Debug)]
pub struct Envelope {
    name: String,
    kind: EnvelopeKind,
}

#[derive(Debug)]
enum EnvelopeKind {
    Income,
    Expense,
}

/// Transactions represent payments in the real world and are associated with an
/// Account and an Envelope .
#[derive(Debug)]
pub struct Transaction {
    payee: String,
    date: NaiveDate,
    kind: TransactionKind,
    amount: Amount,
    envelope: Envelope,
    account: Account,
}

#[derive(Debug)]
enum TransactionKind {
    Withdrawal,
    Deposit,
}

/// A transfer of funds between two Accounts
#[derive(Debug)]
pub struct AccountTransfer {
    source: Account,
    sink: Account,
    amount: Amount,
    date: NaiveDate,
}

/// A transfer of funds between two Envelopes
#[derive(Debug)]
pub struct EnvelopeTransfer {
    source: Envelope,
    sink: Envelope,
    amount: Amount,
    date: NaiveDate,
}

#[derive(Debug)]
pub struct Budget {
    accounts: Vec<Account>,
    envelopes: Vec<Envelope>,
    last_allocation: Option<NaiveDate>,
}

 impl Budget {
    pub fn init() -> io::Result<Self> {
        fs::create_dir("accounts")?;
        fs::create_dir("envelopes")?;
        
        fs::OpenOptions::new().write(true).create_new(true).open("envelope_budget.conf")?;

        let budget = Budget {accounts: Vec::new(), envelopes: Vec::new(), last_allocation: None};
        Ok(budget)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;

    #[test]
    fn init_success() {
        let budget = Budget::init();
        assert!(budget.is_ok());

        fs::remove_dir("accounts");
        fs::remove_dir("envelopes");
        fs::remove_file("envelope_budget.conf");
    }

    #[test]
    fn init_folders_exist() {
        fs::create_dir("accounts");
        fs::create_dir("envelopes");

        let budget = Budget::init();
        assert!(budget.is_err());

        fs::remove_dir("accounts");
        fs::remove_dir("envelopes");
    }

    #[test]
    fn init_config_exist() {
        fs::File::create("envelope_budget.conf");

        let budget = Budget::init();
        assert!(budget.is_err());

        fs::remove_dir("accounts");
        fs::remove_dir("envelopes");
        fs::remove_file("envelope_budget.conf");
    }
}
