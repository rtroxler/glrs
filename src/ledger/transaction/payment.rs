use super::*;

#[derive(Debug)]
pub struct Payment<'a> {
    amount: USD,
    account_code: String,
    pub effective_on: DateTime<Utc>,
    payee_amount: USD,
    payee_account_code: &'a AccountCode,
    payee_service_start_date: Option<DateTime<Utc>>,
    payee_service_end_date: Option<DateTime<Utc>>,
    payee_effective_on: DateTime<Utc>,
    payee_resolved_on: Option<DateTime<Utc>>,
    previously_paid_amount: USD,
    //payee_discount_amount
}

impl<'a> Payment<'a> {
    // TODO I hate this, not having key value args when calling new sucks
    pub fn new( amount: USD, account_code: String, effective_on: DateTime<Utc>, payee_amount: USD,
                payee_account_code: &'a AccountCode, payee_service_start_date: Option<DateTime<Utc>>,
                payee_service_end_date: Option<DateTime<Utc>>, payee_effective_on: DateTime<Utc>,
                payee_resolved_on: Option<DateTime<Utc>>, previously_paid_amount: USD) -> Payment {
        Payment {
            amount: amount,
            account_code: account_code,
            effective_on: effective_on,
            payee_amount: payee_amount,
            payee_account_code: payee_account_code,
            payee_service_start_date: payee_service_start_date,
            payee_service_end_date: payee_service_end_date,
            payee_effective_on: payee_effective_on,
            payee_resolved_on: payee_resolved_on, // Not used at all right now? For credits, which may be an entirely different struct
            previously_paid_amount: previously_paid_amount
        }
    }

    fn process_cash(&self, payee_account_code: &CashAccount, gl: &mut GeneralLedger) {
        gl.record_double_entry(self.effective_on.naive_utc().date(), self.amount, &self.account_code, &payee_account_code.revenue_code);
    }

    fn process_accrual(&self, payee_account_code: &AccrualAccount, gl: &mut GeneralLedger) {
        if self.effective_on >= self.payee_effective_on {
            gl.record_double_entry(self.effective_on.naive_utc().date(), self.amount, &self.account_code, &payee_account_code.accounts_receivable_code);
        } else {
            gl.record_double_entry(self.effective_on.naive_utc().date(), self.amount, &self.account_code, &payee_account_code.deferred_code);
            gl.record_double_entry(self.payee_effective_on.naive_utc().date(), self.amount, &payee_account_code.deferred_code, &payee_account_code.accounts_receivable_code);
        }
    }

    fn process_daily_accrual(&self, payee_account_code: &AccrualAccount, gl: &mut GeneralLedger) {
        // Absolute garbage method name and placement
        let (deferred_amount, leftover_days) = self.record_transaction_date_entries_and_return_deferred(payee_account_code, gl);

        let mut deferred_amount_mut = deferred_amount;
        for (date, amount) in leftover_days {
            if deferred_amount_mut == USD::zero() {
                break;
            }
            if amount <= deferred_amount_mut {
                gl.record_double_entry(date.naive_utc().date(),
                                        amount,
                                        &payee_account_code.deferred_code,
                                        &payee_account_code.accounts_receivable_code);
                deferred_amount_mut -= amount;
            } else {
                gl.record_double_entry(date.naive_utc().date(),
                                        deferred_amount_mut,
                                        &payee_account_code.deferred_code,
                                        &payee_account_code.accounts_receivable_code);
                deferred_amount_mut = USD::zero();
            }
        }
    }
    // This is gross, returning leftover amount and leftover days after recording payment day
    // entries AND it's Daily Accrual specific. Need to have a DA trait maybe? Idk. Does all this
    // belong on a DA account code? since our DA-ness is by account code.
    fn record_transaction_date_entries_and_return_deferred(&self, payee_account_code: &AccrualAccount, gl: &mut GeneralLedger) -> (USD, Vec<(DateTime<Utc>, USD)>) {
        if self.effective_on >= self.payee_service_start_date.unwrap() {
            let days_into_service_period = self.effective_on.signed_duration_since(self.payee_service_start_date.unwrap());
            let mut days_exclusive: usize = (days_into_service_period.to_std().unwrap().as_secs() / 86_400) as usize;

            let amounts = self.payable_amounts_per_day();
            if days_exclusive > amounts.len() {
                days_exclusive = amounts.len();
            }
            let (ar_days, leftover_days) = amounts.split_at(days_exclusive);

            let creditable_ar = ar_days.iter().fold(USD::zero(), |sum, date_amount| sum + date_amount.1);

            // IF I GET CREDITABLE_AR TO WORK WITH PREVIOUSLY PAID, DEFERRED SHOULD JUST WORK
            let (ar_to_credit, deferred_amount) = if self.amount >= creditable_ar {
                (creditable_ar, self.amount - creditable_ar)
            } else {
                (self.amount, USD::zero())
            };
            // payment to ar
            gl.record_double_entry(self.effective_on.naive_utc().date(), ar_to_credit, &self.account_code, &payee_account_code.accounts_receivable_code);

            // payment to deferred if applicable
            if deferred_amount > USD::zero() {
                gl.record_double_entry(self.effective_on.naive_utc().date(), deferred_amount, &self.account_code, &payee_account_code.deferred_code);
            }
            (deferred_amount, leftover_days.to_vec())
        } else {
            // Prepay
            gl.record_double_entry(self.effective_on.naive_utc().date(), self.amount, &self.account_code, &payee_account_code.deferred_code);
            (self.amount, self.payable_amounts_per_day())
        }
    }
}

impl<'a> Transaction for Payment<'a> {
    fn previously_paid_amount(&self) -> USD {
        self.previously_paid_amount
    }
    fn payee_service_start_date(&self) -> Option<DateTime<Utc>> {
        self.payee_service_start_date
    }
    fn payee_service_end_date(&self) -> Option<DateTime<Utc>>  {
        self.payee_service_end_date
    }
    fn payee_amount(&self) -> USD {
        self.payee_amount
    }

    fn process(&self, gl: &mut GeneralLedger) {
        match &self.payee_account_code {
            &&AccountCode::Base(ref _string) => println!("Can't process AC"),
            &&AccountCode::Daily(ref ac) => self.process_daily_accrual(ac, gl),
            &&AccountCode::Periodic(ref ac) => self.process_accrual(ac, gl),
            &&AccountCode::Cash(ref ac) => self.process_cash(ac, gl)
        }
    }
}
