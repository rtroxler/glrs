use usd::USD;

mod general_ledger;
mod transaction;
use ledger::general_ledger::GeneralLedger;
use ledger::transaction::*;

// TODO: serialize chrono so I can replace these
#[derive(Debug, RustcDecodable)]
struct DumbAssessment {
    amount: USD,
    account_code: String,
    effective_on: String,
    service_start_date: String,
    service_end_date: String
}

#[derive(Debug, RustcDecodable)]
struct DumbPayment {
    amount: USD,
    account_code: String,
    effective_on: String,
    service_start_date: String,
    service_end_date: String,
    payee_service_start_date: String,
    payee_service_end_date: String
}

#[derive(Debug, RustcDecodable)]
pub struct Ledger {
    assessments: Vec<DumbAssessment>,
    payments: Vec<DumbPayment>
}

impl Ledger {
    pub fn general_ledger(&self) -> GeneralLedger {
        // TODO
        GeneralLedger::new()
    }
}

