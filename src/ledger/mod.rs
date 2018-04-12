pub mod general_ledger;
pub mod transaction;
use ledger::general_ledger::GeneralLedger;
//use ledger::transaction::*;
use ledger::transaction::assessment::Assessment;
use ledger::transaction::payment::Payment;
use ledger::transaction::Transaction;
use chart_of_accounts::AccountCode;
use chart_of_accounts::ChartOfAccounts;
use chart_of_accounts::CashAccount;
use chart_of_accounts::AccrualAccount;



#[derive(Debug)]
pub struct Ledger<'a> {
    pub assessments: Vec<Assessment<'a>>,
    pub payments: Vec<Payment<'a>>
}

impl<'a> Ledger<'a> {
    pub fn new(assessments: Vec<Assessment<'a>>, payments: Vec<Payment<'a>>) -> Ledger<'a> {
        Ledger {
            assessments: assessments,
            payments: payments
        }
    }

    pub fn chart_of_accounts() -> ChartOfAccounts<'a> {
        let rent = AccountCode::Daily(AccrualAccount {
            revenue_code: String::from("4000"), accounts_receivable_code: String::from("1101"), deferred_code: String::from("2020")
        });
        let fee = AccountCode::Periodic(AccrualAccount {
            revenue_code: String::from("4050"), accounts_receivable_code: String::from("1104"), deferred_code: String::from("")
        });
        let fee2 = AccountCode::Periodic(AccrualAccount {
            revenue_code: String::from("4051"), accounts_receivable_code: String::from("1104"), deferred_code: String::from("")
        });
        let service = AccountCode::Periodic(AccrualAccount {
            revenue_code: String::from("4150"), accounts_receivable_code: String::from("1103"), deferred_code: String::from("2023")
        });
        let insurance = AccountCode::Cash(CashAccount {
            revenue_code: String::from("4100")
        });
        let mut chart = ChartOfAccounts::new();

        chart.table.insert("4000", rent);
        chart.table.insert("4050", fee);
        chart.table.insert("4051", fee2);
        chart.table.insert("4150", service);
        chart.table.insert("4100", insurance);
        chart
    }

    pub fn process_general_ledger(&self) -> GeneralLedger {
        // create chart of accounts
        //chart_of_accounts.play();
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

#[cfg(test)]
mod ledger_tests {
    //use super::*;
    //extern crate chrono;
    //use chrono::prelude::*;
    //use usd::USD;

    #[test]
    fn ledger_can_build_from_an_input_ledger() {
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
