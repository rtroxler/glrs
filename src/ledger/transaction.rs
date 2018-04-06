extern crate chrono;
use chrono::prelude::*;

use usd::USD;

use account_map;

use ledger::general_ledger::GeneralLedger;

// Will not work
// Can't access data without pattern matching it out, which then moves it.
// Not the solution I'm looking for
// enum Transaction  {
// Payment { },
//Assessment { }
//}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assessment {
    amount: USD,
    account_code: String,
    pub effective_on: DateTime<Utc>,
    pub service_start_date: Option<DateTime<Utc>>, // Should really be Date instead
    pub service_end_date: Option<DateTime<Utc>>,
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

#[derive(Debug)]
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
            payee_resolved_on: payee_resolved_on,
            previously_paid_amount: previously_paid_amount
        }
    }
}
//Void?
//Do we need a credit transaction? Can it just be made a part of payee_discount_amount?
//What if it's a full credit, then it would.

pub trait Transaction {
    fn payee_service_start_date(&self) -> Option<DateTime<Utc>>;
    fn payee_service_end_date(&self) -> Option<DateTime<Utc>>;
    fn payee_amount(&self) -> USD;
    fn account_code(&self) -> &str;
    fn previously_paid_amount(&self) -> USD;

    fn days_in_payee_service_period(&self) -> i64 {
        let duration = self.payee_service_end_date().unwrap().signed_duration_since(self.payee_service_start_date().unwrap());
        (duration.to_std().unwrap().as_secs() / 86_400) as i64 + 1
    }

    //// Do we take the closed on and if it's within this period roll it up?
    //// Or not even write it? Maybe this? Other account balances (write off, prorate, etc) would
    //// take care of the rest?
    fn payable_amounts_per_day(&self) -> Vec<(DateTime<Utc>, USD)> {
        // TODO: Worry about negative numbers at some point?
        let spd = self.payee_amount().pennies / self.days_in_payee_service_period();
        let mut leftover = self.payee_amount().pennies % self.days_in_payee_service_period();

        let mut already_paid_amount = self.previously_paid_amount().to_pennies();

        (0..self.days_in_payee_service_period()).map(|day| {
            let mut day_amount = spd;
            if leftover > 0 {
                day_amount += 1;
                leftover -= 1;
            }

            if already_paid_amount > 0 {
                if day_amount <= already_paid_amount {
                    already_paid_amount -= day_amount;
                    day_amount = 0;
                } else {
                    day_amount = already_paid_amount;
                    already_paid_amount = 0;
                }
            }

            (self.payee_service_start_date().unwrap() + chrono::Duration::days(day as i64),
             USD::from_pennies(day_amount) )
        }).collect()
    }

    // Process
    fn process_daily_accrual(&self, gl: &mut GeneralLedger);
    fn process_accrual(&self, gl: &mut GeneralLedger);
    fn process_cash(&self, gl: &mut GeneralLedger);

    fn process(&self, gl: &mut GeneralLedger) {
        // We're assessment (charge), write entries based on our account code
        println!("Processing");
        match self.account_code() {
            "4000" => self.process_daily_accrual(gl),
            "4050" => self.process_accrual(gl),
            "4100" => self.process_cash(gl),
            _ => println!("Fuck")
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

    // TODO: these
    fn process_accrual(&self, _gl: &mut GeneralLedger) {}
    fn process_cash(&self, _gl: &mut GeneralLedger) {}
    fn process_daily_accrual(&self, gl: &mut GeneralLedger) {
        // We're a payment, pay for things

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
        println!("process_daily_accrual");
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

