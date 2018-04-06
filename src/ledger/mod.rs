use usd::USD;

mod general_ledger;
mod transaction;
use ledger::general_ledger::GeneralLedger;
use ledger::transaction::*;

// TODO: serialize chrono so I can replace these
#[derive(Debug, Serialize, Deserialize)]
struct DumbAssessment {
    amount: USD,
    account_code: String,
    effective_on: String,
    service_start_date: String,
    service_end_date: String
}

#[derive(Debug, Serialize, Deserialize)]
struct DumbPayment {
    amount: USD,
    account_code: String,
    effective_on: String,
    service_start_date: String,
    service_end_date: String,
    payee_service_start_date: String,
    payee_service_end_date: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ledger {
    assessments: Vec<Assessment>,
    payments: Vec<DumbPayment>
}

impl Ledger {
    pub fn process_general_ledger(&self) -> GeneralLedger {
        let mut general_ledger = GeneralLedger::new();

        for assessment in &self.assessments {
            assessment.process(&mut general_ledger);
        }

        general_ledger
    }
}

