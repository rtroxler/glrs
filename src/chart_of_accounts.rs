use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub enum AccountCode {
    Base(String),
    Daily(AccrualAccount),
    Periodic(AccrualAccount),
    Cash(CashAccount)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AAccountCode {
    Daily(AccrualAccount),
    Periodic(AccrualAccount),
    Cash(CashAccount)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DumbAss {
    name: String,
    account_code: String
}

impl DumbAss {
    fn process(&self) {
        println!("Dumbasses can't process");
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct DumbLedger {
    asses: Vec<DumbAss>
}
impl DumbLedger {
    fn new() -> DumbLedger {
        let mut l = DumbLedger {
            asses: Vec::new()
        };
        let d_base_ab = DumbAss { name: String::from("he"), account_code: String::from("4000") };
        let a_base_ab = DumbAss { name: String::from("he"), account_code: String::from("4050") };
        let c_base_ab = DumbAss { name: String::from("he"), account_code: String::from("4100") };
        l.asses.push(d_base_ab);
        l.asses.push(a_base_ab);
        l.asses.push(c_base_ab);
        l
    }
}

#[derive(Debug)]
pub struct SmartAss<'a> {
    name: String,
    account_code: &'a AAccountCode
}

impl<'a> SmartAss<'a> {
    fn process(&self) {
        match &self.account_code {
            &&AAccountCode::Daily(ref ac) => println!("Daily accrual ac: {:?}", ac),
            &&AAccountCode::Periodic(ref ac) => println!("Periodic accrual {:?}", ac),
            &&AAccountCode::Cash(ref ac) => println!("Cash {:?}", ac),
        }
    }
}

#[derive(Debug)]
struct SmartLedger<'a> {
    asses: Vec<SmartAss<'a>>
}
impl<'a> SmartLedger<'a> {
    fn new(asses: Vec<SmartAss<'a>>) -> SmartLedger<'a> {
        SmartLedger {
            asses: asses
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccrualAccount {
    pub revenue_code: String,
    pub accounts_receivable_code: String,
    pub deferred_code: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyAccrualAccount {
    pub revenue_code: String,
    pub accounts_receivable_code: String,
    pub deferred_code: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CashAccount {
    pub revenue_code: String
}

#[derive(Debug)]
pub struct ChartOfAccounts<'a> {
    pub table: HashMap<&'a str, AccountCode> //TODO: use str instead?
}

impl<'a> ChartOfAccounts<'a> {
    pub fn new() -> ChartOfAccounts<'a> {
        ChartOfAccounts {
            table: HashMap::new()
        }
    }

    pub fn get(&self, key: &str) -> Option<&AccountCode> {
        self.table.get(key)
    }
}

#[cfg(test)]
mod integration_tests {
    //use super::*;

    #[test]
    fn test_types() {
        //let rent = AAccountCode::Daily(AccrualAccount {
            //revenue_code: String::from("4000"), accounts_receivable_code: String::from("1101"), deferred_code: String::from("2020")
        //});
        //let fee = AAccountCode::Periodic(AccrualAccount {
            //revenue_code: String::from("4050"), accounts_receivable_code: String::from("1104"), deferred_code: String::from("")
        //});
        //let fee2 = AAccountCode::Periodic(AccrualAccount {
            //revenue_code: String::from("4051"), accounts_receivable_code: String::from("1104"), deferred_code: String::from("")
        //});
        //let service = AAccountCode::Periodic(AccrualAccount {
            //revenue_code: String::from("4150"), accounts_receivable_code: String::from("1103"), deferred_code: String::from("2023")
        //});
        //let insurance = AAccountCode::Cash(CashAccount {
            //revenue_code: String::from("4100")
        //});
        //let mut chart = ChartOfAccounts::new();

        //chart.table.insert("4000", &rent);
        //chart.table.insert("4050", &fee);
        //chart.table.insert("4051", &fee2);
        //chart.table.insert("4150", &service);
        //chart.table.insert("4100", &insurance);

        //let ledger = DumbLedger::new();
        //println!("{:?}", ledger);
        //let smart_asses: Vec<SmartAss> = ledger.asses.into_iter().map(|ass|
            //SmartAss {
                //name: ass.name,
                //account_code: &chart.get(&ass.account_code).unwrap()
            //}
        //).collect();
        //let smart_ledger = SmartLedger::new(smart_asses);

        //println!("{:?}", smart_ledger);
    }
}
