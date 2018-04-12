pub mod general_ledger;
mod transaction;
use ledger::general_ledger::GeneralLedger;
//use ledger::transaction::*;
use ledger::transaction::assessment::Assessment;
use ledger::transaction::payment::Payment;
use chart_of_accounts;
use chart_of_accounts::AccountCode;
use chart_of_accounts::CashAccount;
use chart_of_accounts::AccrualAccount;

#[derive(Serialize, Deserialize)]
pub struct Ledger {
    assessments: Vec<Assessment>,
    payments: Vec<Payment>
}

trait AssessmentList { }

impl AssessmentList for Assessment { }

impl Ledger {
    pub fn process_general_ledger(&self) -> GeneralLedger {
        // create chart of accounts
        let chart_of_accounts = chart_of_accounts::ChartOfAccounts::new();
        println!("{:?}", chart_of_accounts);
        chart_of_accounts.play();
        //let mut general_ledger = GeneralLedger::new();
        let general_ledger = GeneralLedger::new();

        // transform assessments / payments and attach their accting_type based on their
        // account_code

        // process should match on accting_type
        //for assessment in &self.assessments {
            //assessment.process(&mut general_ledger);
        //}

        //for payment in &self.payments {
            //payment.process(&mut general_ledger);
        //}

        general_ledger
    }
}

#[cfg(test)]
mod ledger_tests {
    use super::*;
    extern crate chrono;
    use chrono::prelude::*;
    use usd::USD;

    #[test]
    fn ledger_can_build_a_gl() {
        let mut ledger = Ledger { assessments: Vec::new(), payments: Vec::new() };

        let rent_charge = Assessment::new(
            USD::from_float(30.0),
            AccountCode::Daily(AccrualAccount {
               revenue_code: String::from("4000"),
               accounts_receivable_code: String::from("1101"),
               deferred_code: String::from("2020")
            }),
            Utc.ymd(2017, 11, 1).and_hms(0,0,0),
            Some(Utc.ymd(2017, 11, 1).and_hms(0,0,0)),
            Some(Utc.ymd(2017, 11, 30).and_hms(0,0,0)),
            );

        ledger.assessments.push(rent_charge);
        ledger.process_general_ledger();
    }
}
