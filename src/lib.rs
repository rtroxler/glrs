// TODO: Remove me
#![allow(dead_code)]

extern crate libc;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate chrono;
use chrono::prelude::*;

mod ledger;
use ledger::general_ledger::GeneralLedger;
//use ledger::InputLedger;
mod usd;
mod account_map;
mod chart_of_accounts;

mod conversions;

#[no_mangle]
pub extern "C" fn rust_perform(c_ptr: *const libc::c_char) -> *const libc::c_char {
    let string_arg = conversions::string_from_c_ptr(c_ptr);

    let input = InputArgs::from_json(&string_arg);
    let input_ledger = input.ledger;

    // TODO: This shouldn't be on ledger, probably
    let chart = ledger::Ledger::chart_of_accounts();
    let ledger = input_ledger.into_ledger(&chart);

    let result = OutputArg {
        general_ledger: ledger.process_general_ledger()
    };

    let string_result = result.to_json();
    conversions::c_ptr_from_string(&string_result)
}

#[no_mangle]
pub extern "C" fn rust_free(c_ptr: *mut libc::c_void) {
    unsafe {
        libc::free(c_ptr);
    }
}


#[derive(Serialize, Deserialize)]
struct InputArgs {
    ledger: InputLedger
}

impl InputArgs {
    pub fn from_json(json_string: &str) -> InputArgs {
        serde_json::from_str(json_string).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct OutputArg {
    general_ledger: GeneralLedger
}

impl  OutputArg {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}


// TODO Should all of this stuff live elsewhere?
#[derive(Debug, Serialize, Deserialize)]
pub struct InputLedger {
    assessments: Vec<InputAssessment>,
    payments: Vec<InputPayment>
}

impl InputLedger {
    fn into_ledger<'a>(self, chart: &'a chart_of_accounts::ChartOfAccounts) -> ledger::Ledger<'a> {
        let assessments: Vec<ledger::transaction::assessment::Assessment> = self.assessments.into_iter().map(|ass|
            ass.into_assessment(&chart)
        ).collect();
        let payments: Vec<ledger::transaction::payment::Payment> = self.payments.into_iter().map(|payment|
            payment.into_payment(&chart)
        ).collect();

        ledger::Ledger::new(assessments, payments)
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct InputAssessment {
    amount: usd::USD,
    account_code: String,
    pub effective_on: DateTime<Utc>,
    pub service_start_date: Option<DateTime<Utc>>, // TODO Should really be Date instead
    pub service_end_date: Option<DateTime<Utc>>, // TODO Should really be Date instead
}

impl InputAssessment {
    fn into_assessment<'a>(self, chart: &'a chart_of_accounts::ChartOfAccounts) -> ledger::transaction::assessment::Assessment<'a> {
        ledger::transaction::assessment::Assessment::new(
            self.amount,
            &chart.get(&self.account_code).unwrap(), // TODO
            self.effective_on,
            self.service_start_date,
            self.service_end_date,
            )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputPayment {
    amount: usd::USD,
    account_code: String,
    pub effective_on: DateTime<Utc>,
    payee_amount: usd::USD,
    payee_account_code: String,
    payee_service_start_date: Option<DateTime<Utc>>,
    payee_service_end_date: Option<DateTime<Utc>>,
    payee_effective_on: DateTime<Utc>,
    payee_resolved_on: Option<DateTime<Utc>>,
    previously_paid_amount: usd::USD,
}

impl InputPayment {
    fn into_payment<'a>(self, chart: &'a chart_of_accounts::ChartOfAccounts) -> ledger::transaction::payment::Payment<'a> {
        ledger::transaction::payment::Payment::new(
            self.amount,
            self.account_code,
            self.effective_on,
            self.payee_amount,
            &chart.get(&self.payee_account_code).unwrap(), // TODO
            self.payee_service_start_date,
            self.payee_service_end_date,
            self.payee_effective_on,
            self.payee_resolved_on,
            self.previously_paid_amount,
        )
    }
}
