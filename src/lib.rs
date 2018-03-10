extern crate chrono;
use chrono::NaiveDate;

type Amount = i64; // TODO: Money type?

/// An account is a real world banking product where money transactions occur.
/// These transactions will be categorized in to envelopes to be budgeted.
#[derive(Debug)]
pub struct Account {
    name: String,
    classification: AccountClassification
}

#[derive(Debug)]
enum AccountClassification {
    Savings,
    Checking,
    CreditCard,
    Loan,
    Investment,
}

/// An Envelope is a virtual account backed by actual funds in the Accounts . Funds are transferred between
/// Envelopes, often from an Income Envelopes to Expense Envelopes . Transactions are made
/// against Envelopes . This allows each Envelope to have a balance (positive or negative).
#[derive(Debug)]
pub struct Envelope {
    name: String,
    classification: EnvelopeClassification
}

#[derive(Debug)]
enum EnvelopeClassification {
    Income,
    Expense,
}

/// Transactions represent payments in the real world and are associated with an
/// Account and an Envelope .
#[derive(Debug)]
pub struct Transaction {
    payee: String,
    date: NaiveDate,
    classification: TransactionClassification,
    amount: Amount,
    envelope: Envelope,
    account: Account
}

#[derive(Debug)]
enum TransactionClassification {
    Withdrawal,
    Deposit,
}

/// A transfer of funds between two Accounts
#[derive(Debug)]
pub struct AccountTransfer {
    source: Account,
    sink: Account,
    amount: Amount,
    date: NaiveDate
}

/// A transfer of funds between two Envelopes
#[derive(Debug)]
pub struct EnvelopeTransfer {
    source: Envelope,
    sink: Envelope,
    amount: Amount,
    date: NaiveDate
}
