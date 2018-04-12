use super::*;

pub fn daily_accrual_assesment<'a>(chart: &'a ChartOfAccounts) -> Assessment<'a> {
    Assessment::new(
        USD::from_float(30.0),
        chart.get("4000").unwrap(), // should just construct a new one?
        Utc.ymd(2017, 11, 1).and_hms(0,0,0),
        Some(Utc.ymd(2017, 11, 1).and_hms(0,0,0)),
        Some(Utc.ymd(2017, 11, 30).and_hms(0,0,0)),
    )
}

pub fn accrual_assessment<'a>(chart: &'a ChartOfAccounts, code: &str) -> Assessment<'a> {
    Assessment::new(
        USD::from_float(30.0),
        chart.get(code).unwrap(),
        Utc.ymd(2017, 11, 1).and_hms(0,0,0),
        Some(Utc.ymd(2017, 11, 1).and_hms(0,0,0)),
        Some(Utc.ymd(2017, 11, 30).and_hms(0,0,0)),
        )
}

pub fn cash_assessment<'a>(chart: &'a ChartOfAccounts) -> Assessment<'a> {
    Assessment::new(
        USD::from_float(12.0),
        chart.get("4100").unwrap(),
        Utc.ymd(2017, 11, 1).and_hms(0,0,0),
        Some(Utc.ymd(2017, 11, 1).and_hms(0,0,0)),
        Some(Utc.ymd(2017, 11, 30).and_hms(0,0,0)),
    )
}

pub fn basic_payment<'a>(amount: f64, previous_paid_amount: f64, payee_code: &str, chart: &'a ChartOfAccounts) -> Payment<'a> {
    Payment::new(
        USD::from_float(amount),
        String::from("1000"),
        Utc.ymd(2017, 11, 1).and_hms(0,0,0),
        USD::from_float(30.0),
        &chart.get(payee_code).unwrap(),
        Some(Utc.ymd(2017, 11, 1).and_hms(0,0,0)),
        Some(Utc.ymd(2017, 11, 30).and_hms(0,0,0)),
        Utc.ymd(2017,11,1).and_hms(0,0,0), // Is this needed?
        None,
        USD::from_float(previous_paid_amount)
    )
}

pub fn future_payment<'a>(amount: f64, previous_paid_amount: f64, payee_code: &str, chart: &'a ChartOfAccounts) -> Payment<'a> {
    Payment::new(
        USD::from_float(amount),
        String::from("1000"),
        Utc.ymd(2017, 10, 1).and_hms(0,0,0),
        USD::from_float(30.0),
        &chart.get(payee_code).unwrap(),
        Some(Utc.ymd(2017, 11, 1).and_hms(0,0,0)),
        Some(Utc.ymd(2017, 11, 30).and_hms(0,0,0)),
        Utc.ymd(2017,11,1).and_hms(0,0,0), // Is this needed?
        None,
        USD::from_float(previous_paid_amount)
    )
}
