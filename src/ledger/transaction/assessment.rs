use super::*;

#[derive(Debug)]
pub struct Assessment<'a> {
    amount: USD,
    account_code: &'a AccountCode,
    pub effective_on: DateTime<Utc>,
    pub service_start_date: Option<DateTime<Utc>>, // TODO Should really be Date instead
    pub service_end_date: Option<DateTime<Utc>>,   // TODO Should really be Date instead
}

impl<'a> Assessment<'a> {
    pub fn new(
        amount: USD,
        account_code: &'a AccountCode,
        effective_on: DateTime<Utc>,
        service_start_date: Option<DateTime<Utc>>,
        service_end_date: Option<DateTime<Utc>>,
    ) -> Assessment<'a> {
        Assessment {
            amount: amount,
            account_code: account_code,
            effective_on: effective_on,
            service_start_date: service_start_date,
            service_end_date: service_end_date,
        }
    }
}

impl<'a> Transaction<'a> for Assessment<'a> {
    fn previously_paid_amount(&self) -> USD {
        USD::zero()
    }
    fn payee_service_start_date(&self) -> Option<DateTime<Utc>> {
        self.service_start_date
    }
    fn payee_service_end_date(&self) -> Option<DateTime<Utc>> {
        self.service_end_date
    }
    fn payee_amount(&self) -> USD {
        self.amount
    }
    fn process_account_code(&self) -> &'a AccountCode {
        self.account_code
    }

    fn process_daily_accrual(&self, account_code: &AccrualAccount, gl: &mut GeneralLedger) {
        for (date, amount) in self.payable_amounts_per_day() {
            gl.record_double_entry(
                date.naive_utc().date(),
                amount,
                &account_code.accounts_receivable_code,
                &account_code.revenue_code,
            );
        }
    }
    fn process_accrual(&self, account_code: &AccrualAccount, gl: &mut GeneralLedger) {
        gl.record_double_entry(
            self.effective_on.naive_utc().date(),
            self.amount,
            &account_code.accounts_receivable_code,
            &account_code.revenue_code,
        );
    }
    fn process_cash(&self, _account_code: &CashAccount, _gl: &mut GeneralLedger) {}
}
