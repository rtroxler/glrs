use chrono::prelude::*;

use chart_of_accounts::ChartOfAccounts;
use ledger::{
    Ledger,
    transaction::{
        assessment::Assessment,
        payment::Payment
    }
};
use usd::USD;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputLedger {
    assessments: Vec<InputAssessment>,
    payments: Vec<InputPayment>
}

impl InputLedger {
    pub fn into_ledger<'a>(self, chart: &'a ChartOfAccounts) -> Ledger<'a> {
        let assessments: Vec<Assessment> = self.assessments.into_iter().map(|ass|
            ass.into_assessment(&chart)
        ).collect();
        let payments: Vec<Payment> = self.payments.into_iter().map(|payment|
            payment.into_payment(&chart)
        ).collect();

        Ledger::new(assessments, payments)
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct InputAssessment {
    amount: USD,
    account_code: String,
    pub effective_on: DateTime<Utc>,
    pub service_start_date: Option<DateTime<Utc>>, // TODO Should really be Date instead
    pub service_end_date: Option<DateTime<Utc>>, // TODO Should really be Date instead
}

impl InputAssessment {
    fn into_assessment<'a>(self, chart: &'a ChartOfAccounts) -> Assessment<'a> {
        Assessment::new(
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
    amount: USD,
    account_code: String,
    pub effective_on: DateTime<Utc>,
    payee_amount: USD,
    payee_account_code: String,
    payee_service_start_date: Option<DateTime<Utc>>,
    payee_service_end_date: Option<DateTime<Utc>>,
    payee_effective_on: DateTime<Utc>,
    payee_resolved_on: Option<DateTime<Utc>>,
    previously_paid_amount: USD,
}

impl InputPayment {
    fn into_payment<'a>(self, chart: &'a ChartOfAccounts) -> Payment<'a> {
        Payment::new(
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
