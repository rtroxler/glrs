use chrono::prelude::*;

use chart_of_accounts::ChartOfAccounts;
use ledger::{
    Ledger,
    transaction::{
        assessment::Assessment,
        void_assessment::VoidAssessment,
        payment::Payment
    }
};
use usd::USD;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputLedger {
    assessments: Vec<InputAssessment>,
    void_assessments: Vec<InputVoidAssessment>,
    payments: Vec<InputPayment>
}

impl InputLedger {
    pub fn into_ledger<'a>(self, chart: &'a ChartOfAccounts) -> Ledger<'a> {
        let assessments: Vec<Assessment> = self.assessments.into_iter().map(|ass|
            ass.into_assessment(&chart)
        ).collect();
        let void_assessments: Vec<VoidAssessment> = self.void_assessments.into_iter().map(|void_ass|
            void_ass.into_void_assessment(&chart)
        ).collect();
        let payments: Vec<Payment> = self.payments.into_iter().map(|payment|
            payment.into_payment(&chart)
        ).collect();

        Ledger::new(assessments, void_assessments, payments)
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

#[derive(Debug, Serialize, Deserialize)]
pub struct InputVoidAssessment {
    amount: USD,
    pub effective_on: DateTime<Utc>,
    payee_amount: USD,
    payee_account_code: String,
    payee_service_start_date: Option<DateTime<Utc>>,
    payee_service_end_date: Option<DateTime<Utc>>,
    payee_effective_on: DateTime<Utc>,
}

impl InputVoidAssessment {
    fn into_void_assessment<'a>(self, chart: &'a ChartOfAccounts) -> VoidAssessment<'a> {
        VoidAssessment::new(
            self.amount,
            self.effective_on,
            self.payee_amount,
            &chart.get(&self.payee_account_code).unwrap(), // TODO
            self.payee_service_start_date,
            self.payee_service_end_date,
            self.payee_effective_on,
        )
    }
}
