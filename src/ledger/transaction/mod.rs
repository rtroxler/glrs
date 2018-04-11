extern crate chrono;
use chrono::prelude::*;

use usd::USD;
use account_map;
use ledger::general_ledger::GeneralLedger;

pub mod assessment;
pub mod payment;

// Will not work
// Can't access data without pattern matching it out, which then moves it.
// Not the solution I'm looking for
// enum Transaction  {
// Payment { },
//Assessment { }
//}


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
        match self.account_code() {
            "4000" => self.process_daily_accrual(gl),
            "4050" => self.process_accrual(gl),
            "4051" => self.process_accrual(gl),
            "4100" => self.process_cash(gl),
            _ => println!("Fuck")
        }
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use ledger::transaction::assessment::Assessment;
    use ledger::transaction::payment::Payment;

    #[test]
    fn test_rent_account_balance_accrues_daily() {
        let rent_charge = Assessment::new(
            USD::from_float(30.0),
            String::from("4000"),
            Utc.ymd(2017, 11, 1).and_hms(0,0,0),
            Some(Utc.ymd(2017, 11, 1).and_hms(0,0,0)),
            Some(Utc.ymd(2017, 11, 30).and_hms(0,0,0)),
            );

        let mut gl = GeneralLedger::new();
        rent_charge.process(&mut gl);
        let start = rent_charge.service_start_date.unwrap().naive_utc().date();
        let end = rent_charge.service_end_date.unwrap().naive_utc().date();

        let mut date_stepper = start;
        while date_stepper <= end {
            assert_eq!(gl.fetch_amount(date_stepper, String::from("1101")), Some(&USD::from_float(1.0)));
            assert_eq!(gl.fetch_amount(date_stepper, String::from("4000")), Some(&USD::from_float(-1.0)));

            date_stepper = date_stepper.checked_add_signed(chrono::Duration::days(1))
                .expect("Overflow");
        }
    }

    #[test]
    fn test_cash_based_account_balance_records_nothing_on_assessment() {
        let insurance_charge = Assessment::new(
            USD::from_float(12.0),
            String::from("4100"), // Insurance (cash based)
            Utc.ymd(2017, 11, 1).and_hms(0,0,0),
            Some(Utc.ymd(2017, 11, 1).and_hms(0,0,0)),
            Some(Utc.ymd(2017, 11, 30).and_hms(0,0,0)),
            );

        let mut gl = GeneralLedger::new();
        insurance_charge.process(&mut gl);

        assert!(gl.entries().is_empty());
    }

    #[test]
    fn test_cash_based_account_balance_records_entries_on_payment() {
        let insurance_charge = Assessment::new(
            USD::from_float(12.0),
            String::from("4100"), // Insurance (cash based)
            Utc.ymd(2017, 11, 1).and_hms(0,0,0),
            Some(Utc.ymd(2017, 11, 1).and_hms(0,0,0)),
            Some(Utc.ymd(2017, 11, 30).and_hms(0,0,0)),
            );

        let mut gl = GeneralLedger::new();
        insurance_charge.process(&mut gl);

        assert!(gl.entries().is_empty());

        let payment = Payment::new(
            USD::from_float(12.0),
            String::from("1000"),
            Utc.ymd(2017, 11, 1).and_hms(0,0,0),
            USD::from_float(12.0),
            String::from("4100"),
            Some(Utc.ymd(2017, 11, 1).and_hms(0,0,0)),
            Some(Utc.ymd(2017, 11, 30).and_hms(0,0,0)),
            Utc.ymd(2017,11,1).and_hms(0,0,0), // Is this needed?
            None,
            USD::from_float(0.0)
            );

        payment.process(&mut gl);

        assert_eq!(gl.fetch_amount(insurance_charge.effective_on.naive_utc().date(), String::from("1000")), Some(&USD::from_float(12.0)));
        assert_eq!(gl.fetch_amount(insurance_charge.effective_on.naive_utc().date(), String::from("4100")), Some(&USD::from_float(-12.0)));
    }

    #[test]
    fn test_fee_account_balance_accrues_periodically() {
        let fee_charge = Assessment::new(
            USD::from_float(30.0),
            String::from("4050"), // Fee
            Utc.ymd(2017, 11, 1).and_hms(0,0,0),
            None,
            None,
            );

        let mut gl = GeneralLedger::new();
        fee_charge.process(&mut gl);

        assert_eq!(gl.fetch_amount(fee_charge.effective_on.naive_utc().date(), String::from("1103")), Some(&USD::from_float(30.0)));
        assert_eq!(gl.fetch_amount(fee_charge.effective_on.naive_utc().date(), String::from("4050")), Some(&USD::from_float(-30.0)));

        // Doesn't have anything the next day
        // assert entries count == 2
    }

    #[test]
    fn test_fee_account_balance_accrues_periodically_and_handles_payment() {
        let fee_charge = Assessment::new(
            USD::from_float(30.0),
            String::from("4050"), // Fee
            Utc.ymd(2017, 11, 1).and_hms(0,0,0),
            None,
            None,
            );

        let payment = Payment::new(
            USD::from_float(30.0),
            String::from("1000"),
            Utc.ymd(2017, 11, 1).and_hms(0,0,0),
            USD::from_float(30.0),
            String::from("4050"),
            None,
            None,
            Utc.ymd(2017,11,1).and_hms(0,0,0), // Is this needed?
            None,
            USD::from_float(0.0)
            );

        let mut gl = GeneralLedger::new();
        fee_charge.process(&mut gl);
        payment.process(&mut gl);

        assert_eq!(gl.fetch_amount(fee_charge.effective_on.naive_utc().date(), String::from("1000")), Some(&USD::from_float(30.0)));
        assert_eq!(gl.fetch_amount(fee_charge.effective_on.naive_utc().date(), String::from("4050")), Some(&USD::from_float(-30.0)));

        // Doesn't have anything the next day
        // assert entries count == 2
    }

    #[test]
    fn test_a_full_payment_against_rent() {
        let mut gl = GeneralLedger::new();

        let rent_charge = Assessment::new(
            USD::from_float(30.0),
            String::from("4000"),
            Utc.ymd(2017, 11, 1).and_hms(0,0,0),
            Some(Utc.ymd(2017, 11, 1).and_hms(0,0,0)),
            Some(Utc.ymd(2017, 11, 30).and_hms(0,0,0)),
            );

        let payment = Payment::new(
            USD::from_float(30.0),
            String::from("1000"),
            Utc.ymd(2017, 11, 1).and_hms(0,0,0),
            USD::from_float(30.0),
            String::from("4000"),
            Some(Utc.ymd(2017, 11, 1).and_hms(0,0,0)),
            Some(Utc.ymd(2017, 11, 30).and_hms(0,0,0)),
            Utc.ymd(2017,11,1).and_hms(0,0,0), // Is this needed?
            None,
            USD::from_float(0.0)
            );

        rent_charge.process(&mut gl);
        payment.process(&mut gl);

        assert_eq!(gl.fetch_amount(payment.effective_on.naive_utc().date(), String::from("1000")), Some(&USD::from_float(30.0)));
        assert_eq!(gl.fetch_amount(payment.effective_on.naive_utc().date(), String::from("2020")), Some(&USD::from_float(-29.0)));
        assert_eq!(gl.fetch_amount(payment.effective_on.naive_utc().date(), String::from("4000")), Some(&USD::from_float(-1.0)));

        let start = rent_charge.service_start_date.unwrap().naive_utc().date();
        let end = rent_charge.service_end_date.unwrap().naive_utc().date();
        let mut date_stepper = start.checked_add_signed(chrono::Duration::days(1)).expect("Overflow");
        while date_stepper <= end {
            assert_eq!(gl.fetch_amount(date_stepper, String::from("2020")), Some(&USD::from_float(1.0)));
            assert_eq!(gl.fetch_amount(date_stepper, String::from("4000")), Some(&USD::from_float(-1.0)));

            date_stepper = date_stepper.checked_add_signed(chrono::Duration::days(1))
                .expect("Overflow");
        }
    }

    #[test]
    fn test_two_even_partial_payments_against_rent() {
        let mut gl = GeneralLedger::new();

        let rent_charge = Assessment::new(
            USD::from_float(30.0),
            String::from("4000"),
            Utc.ymd(2017, 11, 1).and_hms(0,0,0),
            Some(Utc.ymd(2017, 11, 1).and_hms(0,0,0)),
            Some(Utc.ymd(2017, 11, 30).and_hms(0,0,0)),
            );

        let payment1 = Payment::new(
            USD::from_float(15.0),
            String::from("1000"),
            Utc.ymd(2017, 11, 1).and_hms(0,0,0),
            USD::from_float(30.0),
            String::from("4000"),
            Some(Utc.ymd(2017, 11, 1).and_hms(0,0,0)),
            Some(Utc.ymd(2017, 11, 30).and_hms(0,0,0)),
            Utc.ymd(2017,11,1).and_hms(0,0,0), // Is this needed?
            None,
            USD::from_float(0.0)
            );

        let payment2 = Payment::new(
            USD::from_float(15.0),
            String::from("1000"),
            Utc.ymd(2017, 11, 1).and_hms(0,0,0),
            USD::from_float(30.0),
            String::from("4000"),
            Some(Utc.ymd(2017, 11, 1).and_hms(0,0,0)),
            Some(Utc.ymd(2017, 11, 30).and_hms(0,0,0)),
            Utc.ymd(2017,11,1).and_hms(0,0,0), // Is this needed?
            None,
            USD::from_float(15.0)
            );

        rent_charge.process(&mut gl);
        payment1.process(&mut gl);
        payment2.process(&mut gl);

        assert_eq!(gl.fetch_amount(payment1.effective_on.naive_utc().date(), String::from("1000")), Some(&USD::from_float(30.0)));
        assert_eq!(gl.fetch_amount(payment1.effective_on.naive_utc().date(), String::from("2020")), Some(&USD::from_float(-29.0)));
        assert_eq!(gl.fetch_amount(payment1.effective_on.naive_utc().date(), String::from("4000")), Some(&USD::from_float(-1.0)));

        let start = rent_charge.service_start_date.unwrap().naive_utc().date();
        let end = rent_charge.service_end_date.unwrap().naive_utc().date();
        let mut date_stepper = start.checked_add_signed(chrono::Duration::days(1)).expect("Overflow");
        while date_stepper <= end {
            assert_eq!(gl.fetch_amount(date_stepper, String::from("2020")), Some(&USD::from_float(1.0)));
            assert_eq!(gl.fetch_amount(date_stepper, String::from("4000")), Some(&USD::from_float(-1.0)));

            date_stepper = date_stepper.checked_add_signed(chrono::Duration::days(1))
                .expect("Overflow");
        }
    }


    // TODO
    // payments
    // 15 + 15 upfront
    // 15 + 15 with second on day 20
    // 15.5 + 14.5
    // 15 + 15, void the first
    // void in general
    //
    // credits
    //
    // move out / rental termination
}
