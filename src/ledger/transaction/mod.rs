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

pub trait Transaction {
    fn payee_service_start_date(&self) -> Option<DateTime<Utc>>;
    fn payee_service_end_date(&self) -> Option<DateTime<Utc>>;
    fn payee_amount(&self) -> USD;
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

    // TODO: Make an account code fetcher again
    fn process(&self, _gl: &mut GeneralLedger) {
        println!("wat");
        // We're assessment (charge), write entries based on our account code
        //self.process_daily_accrual(gl) // TODO no
        //match self.account_code() {
            //"4000" => self.process_daily_accrual(gl),
            //"4050" => self.process_accrual(gl),
            //"4051" => self.process_accrual(gl),
            //"4100" => self.process_cash(gl),
            //"4150" => self.process_accrual(gl),
            //_ => println!("Fuck")
        //}
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use ledger::transaction::assessment::Assessment;
    use ledger::transaction::payment::Payment;
    use chart_of_accounts::ChartOfAccounts;

    #[test]
    fn test_rent_account_balance_accrues_daily() {
        let chart = ChartOfAccounts::cubesmart();

        println!("setup");
        let rent_charge = Assessment::new(
            USD::from_float(30.0),
            &chart.get("4000").unwrap(),
            Utc.ymd(2017, 11, 1).and_hms(0,0,0),
            Some(Utc.ymd(2017, 11, 1).and_hms(0,0,0)),
            Some(Utc.ymd(2017, 11, 30).and_hms(0,0,0)),
            );

        println!("setup");
        let mut gl = GeneralLedger::new();
        println!("process");
        rent_charge.process(&mut gl);
        println!("process done");
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
        let chart = ChartOfAccounts::cubesmart();
        let insurance_charge = Assessment::new(
            USD::from_float(12.0),
            &chart.get("4100").unwrap(),
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
        let chart = ChartOfAccounts::cubesmart();
        let insurance_charge = Assessment::new(
            USD::from_float(12.0),
            &chart.get("4100").unwrap(),
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
            &chart.get("4100").unwrap(),
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
        let chart = ChartOfAccounts::cubesmart();
        let fee_charge = Assessment::new(
            USD::from_float(30.0),
            &chart.get("4050").unwrap(),
            Utc.ymd(2017, 11, 1).and_hms(0,0,0),
            None,
            None,
            );

        let mut gl = GeneralLedger::new();
        fee_charge.process(&mut gl);

        assert_eq!(gl.fetch_amount(fee_charge.effective_on.naive_utc().date(), String::from("1104")), Some(&USD::from_float(30.0)));
        assert_eq!(gl.fetch_amount(fee_charge.effective_on.naive_utc().date(), String::from("4050")), Some(&USD::from_float(-30.0)));

        // Doesn't have anything the next day
        // assert entries count == 2
    }

    #[test]
    fn test_fee_account_balance_accrues_and_is_paid_periodically() {
        let chart = ChartOfAccounts::cubesmart();
        let fee_charge = Assessment::new(
            USD::from_float(30.0),
            &chart.get("4050").unwrap(),
            Utc.ymd(2017, 11, 1).and_hms(0,0,0),
            None,
            None,
            );

        let payment = Payment::new(
            USD::from_float(30.0),
            String::from("1000"),
            Utc.ymd(2017, 11, 1).and_hms(0,0,0),
            USD::from_float(30.0),
            &chart.get("4050").unwrap(),
            None,
            None,
            Utc.ymd(2017,11,1).and_hms(0,0,0), // Is this needed?
            None,
            USD::from_float(0.0)
            );

        let mut gl = GeneralLedger::new();
        fee_charge.process(&mut gl);
        payment.process(&mut gl);
        gl.print();

        assert_eq!(gl.fetch_amount(fee_charge.effective_on.naive_utc().date(), String::from("1000")), Some(&USD::from_float(30.0)));
        assert_eq!(gl.fetch_amount(fee_charge.effective_on.naive_utc().date(), String::from("4050")), Some(&USD::from_float(-30.0)));

        // Doesn't have anything the next day
        // assert entries count == 2
    }

    #[test]
    fn test_a_full_payment_against_rent() {
        let chart = ChartOfAccounts::cubesmart();
        let mut gl = GeneralLedger::new();

        let rent_charge = Assessment::new(
            USD::from_float(30.0),
            &chart.get("4000").unwrap(),
            Utc.ymd(2017, 11, 1).and_hms(0,0,0),
            Some(Utc.ymd(2017, 11, 1).and_hms(0,0,0)),
            Some(Utc.ymd(2017, 11, 30).and_hms(0,0,0)),
            );

        let payment = Payment::new(
            USD::from_float(30.0),
            String::from("1000"),
            Utc.ymd(2017, 11, 1).and_hms(0,0,0),
            USD::from_float(30.0),
            &chart.get("4000").unwrap(),
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
    fn test_a_full_payment_against_future_daily_accrual_assessment() {
        let chart = ChartOfAccounts::cubesmart();
        let mut gl = GeneralLedger::new();

        let rent_charge = Assessment::new(
            USD::from_float(30.0),
            &chart.get("4000").unwrap(),
            Utc.ymd(2017, 11, 1).and_hms(0,0,0),
            Some(Utc.ymd(2017, 11, 1).and_hms(0,0,0)),
            Some(Utc.ymd(2017, 11, 30).and_hms(0,0,0)),
            );

        let payment = Payment::new(
            USD::from_float(30.0),
            String::from("1000"),
            Utc.ymd(2017, 10, 1).and_hms(0,0,0),
            USD::from_float(30.0),
            &chart.get("4000").unwrap(),
            Some(Utc.ymd(2017, 11, 1).and_hms(0,0,0)),
            Some(Utc.ymd(2017, 11, 30).and_hms(0,0,0)),
            Utc.ymd(2017,11,1).and_hms(0,0,0), // Is this needed?
            None,
            USD::from_float(0.0)
            );

        rent_charge.process(&mut gl);
        payment.process(&mut gl);

        assert_eq!(gl.fetch_amount(payment.effective_on.naive_utc().date(), String::from("1000")), Some(&USD::from_float(30.0)));
        assert_eq!(gl.fetch_amount(payment.effective_on.naive_utc().date(), String::from("2020")), Some(&USD::from_float(-30.0)));

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
    fn test_a_full_payment_against_future_accrual_assessment() {
        let chart = ChartOfAccounts::cubesmart();
        let mut gl = GeneralLedger::new();

        let service_charge = Assessment::new(
            USD::from_float(30.0),
            &chart.get("4150").unwrap(),
            Utc.ymd(2017, 11, 1).and_hms(0,0,0),
            Some(Utc.ymd(2017, 11, 1).and_hms(0,0,0)),
            Some(Utc.ymd(2017, 11, 30).and_hms(0,0,0)),
            );

        let payment = Payment::new(
            USD::from_float(30.0),
            String::from("1000"),
            Utc.ymd(2017, 10, 1).and_hms(0,0,0),
            USD::from_float(30.0),
            &chart.get("4150").unwrap(),
            Some(Utc.ymd(2017, 11, 1).and_hms(0,0,0)),
            Some(Utc.ymd(2017, 11, 30).and_hms(0,0,0)),
            Utc.ymd(2017,11,1).and_hms(0,0,0), // Is this needed?
            None,
            USD::from_float(0.0)
            );

        service_charge.process(&mut gl);
        payment.process(&mut gl);

        assert_eq!(gl.fetch_amount(payment.effective_on.naive_utc().date(), String::from("1000")), Some(&USD::from_float(30.0)));
        assert_eq!(gl.fetch_amount(payment.effective_on.naive_utc().date(), String::from("2023")), Some(&USD::from_float(-30.0)));

        assert_eq!(gl.fetch_amount(service_charge.effective_on.naive_utc().date(), String::from("2023")), Some(&USD::from_float(30.0)));
        assert_eq!(gl.fetch_amount(service_charge.effective_on.naive_utc().date(), String::from("4150")), Some(&USD::from_float(-30.0)));
    }


    #[test]
    fn test_two_even_partial_payments_against_rent() {
        let chart = ChartOfAccounts::cubesmart();
        let mut gl = GeneralLedger::new();

        let rent_charge = Assessment::new(
            USD::from_float(30.0),
            &chart.get("4000").unwrap(),
            Utc.ymd(2017, 11, 1).and_hms(0,0,0),
            Some(Utc.ymd(2017, 11, 1).and_hms(0,0,0)),
            Some(Utc.ymd(2017, 11, 30).and_hms(0,0,0)),
            );

        let payment1 = Payment::new(
            USD::from_float(15.0),
            String::from("1000"),
            Utc.ymd(2017, 11, 1).and_hms(0,0,0),
            USD::from_float(30.0),
            &chart.get("4000").unwrap(),
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
            &chart.get("4000").unwrap(),
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
    // refunds
    //
    // move out / rental termination
}
