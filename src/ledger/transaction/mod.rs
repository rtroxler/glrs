extern crate chrono;
use chrono::prelude::*;

use usd::USD;
use ledger::general_ledger::GeneralLedger;
use chart_of_accounts::AccountCode;
use chart_of_accounts::CashAccount;
use chart_of_accounts::AccrualAccount;

pub mod assessment;
pub mod payment;

//Void?
//Do we need a credit transaction? Can it just be made a part of payee_discount_amount?
//What if it's a full credit, then it would.

pub trait Transaction<'a> {
    fn payee_service_start_date(&self) -> Option<DateTime<Utc>>;
    fn payee_service_end_date(&self) -> Option<DateTime<Utc>>;
    fn payee_amount(&self) -> USD;
    fn previously_paid_amount(&self) -> USD;
    fn process_account_code(&self) -> &'a AccountCode;

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

    fn process(&self, gl: &mut GeneralLedger) {
        match self.process_account_code() {
            &AccountCode::Base(ref _string) => println!("Can't process AC"),
            &AccountCode::Daily(ref ac) => self.process_daily_accrual(ac, gl),
            &AccountCode::Periodic(ref ac) => self.process_accrual(ac, gl),
            &AccountCode::Cash(ref ac) => self.process_cash(ac, gl),
        }
    }

    fn process_cash(&self, payee_account_code: &CashAccount, gl: &mut GeneralLedger);
    fn process_accrual(&self, payee_account_code: &AccrualAccount, gl: &mut GeneralLedger);
    fn process_daily_accrual(&self, payee_account_code: &AccrualAccount, gl: &mut GeneralLedger);
}

#[cfg(test)]
mod tests {

}
