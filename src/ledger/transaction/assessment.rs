use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Assessment {
    amount: USD,
    account_code: String,
    pub effective_on: DateTime<Utc>,
    pub service_start_date: Option<DateTime<Utc>>, // TODO Should really be Date instead
    pub service_end_date: Option<DateTime<Utc>>, // TODO Should really be Date instead
}

impl Assessment {
    pub fn new(amount: USD, account_code: String, effective_on: DateTime<Utc>,
               service_start_date: Option<DateTime<Utc>>, service_end_date: Option<DateTime<Utc>>) -> Assessment {
        Assessment {
            amount: amount,
            account_code: account_code,
            effective_on: effective_on,
            service_start_date: service_start_date,
            service_end_date: service_end_date
        }
    }
}

impl Transaction for Assessment {
    fn account_code(&self) -> &str {
        self.account_code.as_str()
    }
    fn previously_paid_amount(&self) -> USD {
        USD::zero()
    }
    fn payee_service_start_date(&self) -> Option<DateTime<Utc>> {
        self.service_start_date
    }
    fn payee_service_end_date(&self) -> Option<DateTime<Utc>>  {
        self.service_end_date
    }
    fn payee_amount(&self) -> USD {
        self.amount
    }

    fn process_daily_accrual(&self, gl: &mut GeneralLedger) {
        // We're assessment (charge), write entries based on our account code
        for (date, amount) in self.payable_amounts_per_day() {
            gl.record_double_entry(date.naive_utc().date(),
                                   amount,
                                   &account_map::accounts_receivable_code(&self.account_code),
                                   &self.account_code);
        }

    }

    fn process_accrual(&self, gl: &mut GeneralLedger) {
        gl.record_double_entry(self.effective_on.naive_utc().date(),
                               self.amount,
                               &account_map::accounts_receivable_code(&self.account_code),
                               &self.account_code);
    }

    fn process_cash(&self, _gl: &mut GeneralLedger) {
        // Do nothing
    }
}
