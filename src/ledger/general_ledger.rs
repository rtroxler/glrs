extern crate chrono;
use chrono::prelude::*;
use std::collections::HashMap;
use std::collections::BTreeMap;

extern crate serde;
extern crate serde_json;

use usd::USD;

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneralLedger { // By Day
    entries: HashMap<(NaiveDate, String), USD>
}

impl GeneralLedger {
    pub fn new() -> GeneralLedger {
        GeneralLedger {
            entries: HashMap::new()
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn print(&self) {
        // TODO: This is turrible
        println!("|    Date    | Acct | Debit | Credit |");
        println!("--------------------------------------");
        let ordered: BTreeMap<_, _>  = self.entries.iter().collect();
        for (&(date, ref code), amount) in ordered {
            if amount.pennies > 0 {
                println!("| {} | {} | {:?} |       |", date, code, amount);
            } else if amount.pennies < 0 {
                println!("| {} | {} |       | {:?} |", date, code, amount.inverse());
            } else {
                println!("| {} | {} |       |      |", date, code);
            }
        }
    }

    pub fn record_double_entry(&mut self, date: NaiveDate, amount: USD,
                           debit_account_code: &String, credit_account_code: &String) {
        {
            let debit = self.entries.entry((date, debit_account_code.clone())).or_insert(USD::zero());
            *debit += amount;
        }
        {
            let credit = self.entries.entry((date, credit_account_code.clone())).or_insert(USD::zero());
            *credit -= amount;
        }
    }

    pub fn fetch_amount(&self, date: NaiveDate, account_code: String) -> Option<&USD> {
        self.entries.get(&(date, account_code))
    }
}
