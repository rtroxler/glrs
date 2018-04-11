pub mod general_ledger;
mod transaction;
use ledger::general_ledger::GeneralLedger;
use ledger::transaction::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Ledger {
    assessments: Vec<Assessment>,
    payments: Vec<Payment>
}

impl Ledger {
    pub fn process_general_ledger(&self) -> GeneralLedger {
        let mut general_ledger = GeneralLedger::new();

        for assessment in &self.assessments {
            assessment.process(&mut general_ledger);
        }

        for payment in &self.payments {
            payment.process(&mut general_ledger);
        }

        general_ledger
    }
}

