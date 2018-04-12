extern crate glrs;
use glrs::ledger::{
    transaction::assessment::Assessment,
    transaction::payment::Payment,
    transaction::Transaction,
    general_ledger::GeneralLedger
};
use glrs::usd::USD;

extern crate chrono;
use chrono::prelude::*;
use glrs::chart_of_accounts::ChartOfAccounts;

#[test]
fn it_tests_things() {
    println!("hello world");
}

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
