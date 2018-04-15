pub mod general_ledger;
pub mod transaction;

use ledger::{
    transaction::assessment::Assessment,
    transaction::void_assessment::VoidAssessment,
    transaction::payment::Payment,
    transaction::Transaction,
    general_ledger::GeneralLedger
};



#[derive(Debug)]
pub struct Ledger<'a> {
    pub assessments: Vec<Assessment<'a>>,
    pub void_assessments: Vec<VoidAssessment<'a>>,
    pub payments: Vec<Payment<'a>>
}

impl<'a> Ledger<'a> {
    pub fn new(assessments: Vec<Assessment<'a>>, void_assessments: Vec<VoidAssessment<'a>>, payments: Vec<Payment<'a>>) -> Ledger<'a> {
        Ledger {
            assessments: assessments,
            void_assessments: void_assessments,
            payments: payments
        }
    }

    pub fn process_general_ledger(&self) -> GeneralLedger {
        let mut general_ledger = GeneralLedger::new();

        for assessment in &self.assessments {
            assessment.process(&mut general_ledger);
        }

        for void_assessment in &self.void_assessments {
            void_assessment.process(&mut general_ledger);
        }

        for payment in &self.payments {
            payment.process(&mut general_ledger);
        }

        general_ledger
    }
}

#[cfg(test)]
mod ledger_tests {
    //use super::*;
    //extern crate chrono;
    //use chrono::prelude::*;
    //use usd::USD;
    //use chart_of_accounts::{AccountCode, ChartOfAccounts, CashAccount, AccrualAccount};

    #[test]
    fn ledger_can_build_from_an_input_ledger() {
        // TODO
    }

    #[test]
    fn ledger_can_build_a_gl() {
        //let mut ledger = Ledger { assessments: Vec::new(), payments: Vec::new() };

        //let rent_charge = Assessment::new(
            //USD::from_float(30.0),
            //&AccountCode::Daily(AccrualAccount {
               //revenue_code: String::from("4000"),
               //accounts_receivable_code: String::from("1101"),
               //deferred_code: String::from("2020")
            //}),
            //Utc.ymd(2017, 11, 1).and_hms(0,0,0),
            //Some(Utc.ymd(2017, 11, 1).and_hms(0,0,0)),
            //Some(Utc.ymd(2017, 11, 30).and_hms(0,0,0)),
            //);

        //ledger.assessments.push(rent_charge);
        //ledger.process_general_ledger();
    }
}
