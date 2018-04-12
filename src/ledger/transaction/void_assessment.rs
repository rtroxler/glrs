use super::*;

#[derive(Debug)]
pub struct VoidAssessment<'a> {
    amount: USD,
    pub effective_on: DateTime<Utc>,
    payee_amount: USD,
    payee_account_code: &'a AccountCode,
    payee_service_start_date: Option<DateTime<Utc>>,
    payee_service_end_date: Option<DateTime<Utc>>,
    payee_effective_on: DateTime<Utc>,
}

impl<'a> VoidAssessment<'a> {
    pub fn new( amount: USD, effective_on: DateTime<Utc>, payee_amount: USD,
                payee_account_code: &'a AccountCode, payee_service_start_date: Option<DateTime<Utc>>,
                payee_service_end_date: Option<DateTime<Utc>>, payee_effective_on: DateTime<Utc>) -> VoidAssessment {
        VoidAssessment {
            amount: amount,
            effective_on: effective_on,
            payee_amount: payee_amount,
            payee_account_code: payee_account_code,
            payee_service_start_date: payee_service_start_date,
            payee_service_end_date: payee_service_end_date,
            payee_effective_on: payee_effective_on,
        }
    }
}

impl<'a> Transaction<'a> for VoidAssessment<'a> {
    fn previously_paid_amount(&self) -> USD {
        USD::zero()
    }
    fn payee_service_start_date(&self) -> Option<DateTime<Utc>> {
        self.payee_service_start_date
    }
    fn payee_service_end_date(&self) -> Option<DateTime<Utc>>  {
        self.payee_service_end_date
    }
    fn payee_amount(&self) -> USD {
        self.amount
    }
    fn process_account_code(&self) -> &'a AccountCode {
        self.payee_account_code
    }

    fn process_daily_accrual(&self, account_code: &AccrualAccount, gl: &mut GeneralLedger) {
        for (date, amount) in self.payable_amounts_per_day() {
            let safe_entry_date = if date < self.effective_on { self.effective_on } else { date };

            gl.record_double_entry(safe_entry_date.naive_utc().date(),
                                amount,
                                &account_code.accounts_receivable_code,
                                &account_code.revenue_code);
        }
    }

    fn process_accrual(&self, account_code: &AccrualAccount, gl: &mut GeneralLedger) {
        gl.record_double_entry(self.effective_on.naive_utc().date(),
                               self.amount,
                               &account_code.accounts_receivable_code,
                               &account_code.revenue_code);
    }

    fn process_cash(&self, _account_code: &CashAccount, _gl: &mut GeneralLedger) {}
}
