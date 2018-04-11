use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Payment {
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
    //payee_discount_amount
}

impl Payment {
    pub fn new( amount: USD, account_code: String, effective_on: DateTime<Utc>, payee_amount: USD,
                payee_account_code: String, payee_service_start_date: Option<DateTime<Utc>>,
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
}

impl Transaction for Payment {
    fn account_code(&self) -> &str {
        self.payee_account_code.as_str()
    }
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

    fn process_accrual(&self, gl: &mut GeneralLedger) {
        // TODO; Does not defer if before start date
        gl.record_double_entry(self.effective_on.naive_utc().date(), self.amount, &self.account_code, &account_map::accounts_receivable_code(&self.payee_account_code));
    }

    // TODO: these
    fn process_cash(&self, _gl: &mut GeneralLedger) {}
    fn process_daily_accrual(&self, gl: &mut GeneralLedger) {
        // For Credits
        if self.account_code == String::from("4501") {
            match self.payee_resolved_on {
                Some(_date) => println!("Payee is resolved! Do credit things"), // The entries for this might be weird when paired with a payment. But YOLO
                None => return,
            }
        }

        // TODO: Work previously paid in here
        // How much to A/R?
        let days_into_service_period = self.effective_on.signed_duration_since(self.payee_service_start_date.unwrap());
        let mut days_exclusive: usize = (days_into_service_period.to_std().unwrap().as_secs() / 86_400) as usize;
        //println!("Days exclusive: {:?}", days_exclusive);

        let amounts = self.payable_amounts_per_day();
        //println!("{:?}", amounts);
        if days_exclusive > amounts.len() {
            days_exclusive = amounts.len();
        }
        let (ar_days, leftover_days) = amounts.split_at(days_exclusive);
        //println!("A/R days: {:?}", ar_days);
        //println!("Leftover days: {:?}", leftover_days);

        let creditable_ar = ar_days.iter().fold(USD::zero(), |sum, date_amount| sum + date_amount.1);
        //println!("AR to credit: {:?}", creditable_ar);

        // IF I GET CREDITABLE_AR TO WORK WITH PREVIOUSLY PAID, DEFERRED SHOULD JUST WORK
        let (ar_to_credit, deferred_amount) = if self.amount >= creditable_ar {
            (creditable_ar, self.amount - creditable_ar)
        } else {
            (self.amount, USD::zero())
        };
        // payment to ar
        gl.record_double_entry(self.effective_on.naive_utc().date(), ar_to_credit, &self.account_code, &account_map::accounts_receivable_code(&self.payee_account_code));

        // payment to deferred if applicable
        if deferred_amount > USD::zero() {
            //println!("Deferred amount {:?}", deferred_amount);
            gl.record_double_entry(self.effective_on.naive_utc().date(), deferred_amount, &self.account_code, &account_map::deferred_code(&self.payee_account_code));
        }

        // Need to "eat" previously paid first.
        let mut deferred_amount_mut = deferred_amount;
        for &(date, amount) in leftover_days {
            if deferred_amount_mut == USD::zero() {
                break;
            }
            if amount <= deferred_amount_mut {
                gl.record_double_entry(date.naive_utc().date(),
                                        amount,
                                        &account_map::deferred_code(&self.payee_account_code),
                                        &account_map::accounts_receivable_code(&self.payee_account_code));
                deferred_amount_mut -= amount;
            } else {
                gl.record_double_entry(date.naive_utc().date(),
                                        deferred_amount_mut,
                                        &account_map::deferred_code(&self.payee_account_code),
                                        &account_map::accounts_receivable_code(&self.payee_account_code));
                deferred_amount_mut = USD::zero();
            }
        }
    }
}
