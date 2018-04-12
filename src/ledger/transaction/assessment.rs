use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Assessment {
    amount: USD,
    account_code: AccountCode,
    pub effective_on: DateTime<Utc>,
    pub service_start_date: Option<DateTime<Utc>>, // TODO Should really be Date instead
    pub service_end_date: Option<DateTime<Utc>>, // TODO Should really be Date instead
}

impl Assessment {
    pub fn new(amount: USD, account_code: AccountCode, effective_on: DateTime<Utc>,
               service_start_date: Option<DateTime<Utc>>, service_end_date: Option<DateTime<Utc>>) -> Assessment {
        Assessment {
            amount: amount,
            account_code: account_code,
            effective_on: effective_on,
            service_start_date: service_start_date,
            service_end_date: service_end_date
        }
    }

    fn process_daily_accrual(&self, account_code: &AccrualAccount, gl: &mut GeneralLedger) {
        // We're assessment (charge), write entries based on our account code
        for (date, amount) in self.payable_amounts_per_day() {
            gl.record_double_entry(date.naive_utc().date(),
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
}

impl Transaction for Assessment {
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
    fn process(&self, gl: &mut GeneralLedger) {
        match &self.account_code {
            &AccountCode::Base(ref string) => println!("Can't process AC"),
            &AccountCode::Daily(ref ac) => self.process_daily_accrual(ac, gl),
            &AccountCode::Periodic(ref ac) => self.process_accrual(ac, gl),
            &AccountCode::Cash(ref ac) => println!("Cash {:?}", ac),
        }
    }
}
