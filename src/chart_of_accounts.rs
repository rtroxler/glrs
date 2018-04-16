use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub enum AccountCode {
    Base(String),
    Daily(AccrualAccount),
    Periodic(AccrualAccount),
    Cash(CashAccount)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccrualAccount {
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
    pub table: HashMap<&'a str, AccountCode>
}

impl<'a> ChartOfAccounts<'a> {
    pub fn new() -> ChartOfAccounts<'a> {
        ChartOfAccounts {
            table: HashMap::new()
        }
    }

    pub fn cubesmart() -> ChartOfAccounts<'a> {
        let rent = AccountCode::Daily(AccrualAccount {
            revenue_code: String::from("4000"), accounts_receivable_code: String::from("1101"), deferred_code: String::from("2020")
        });
        let fee = AccountCode::Periodic(AccrualAccount {
            revenue_code: String::from("4050"), accounts_receivable_code: String::from("1104"), deferred_code: String::from("")
        });
        let fee2 = AccountCode::Periodic(AccrualAccount {
            revenue_code: String::from("4051"), accounts_receivable_code: String::from("1104"), deferred_code: String::from("")
        });
        let service = AccountCode::Periodic(AccrualAccount {
            revenue_code: String::from("4150"), accounts_receivable_code: String::from("1103"), deferred_code: String::from("2023")
        });
        let insurance = AccountCode::Cash(CashAccount {
            revenue_code: String::from("4100")
        });
        let mut chart = ChartOfAccounts::new();

        chart.table.insert("4000", rent);
        chart.table.insert("4050", fee);
        chart.table.insert("4051", fee2);
        chart.table.insert("4150", service);
        chart.table.insert("4100", insurance);
        chart
    }

    pub fn get(&self, key: &str) -> Option<&AccountCode> {
        self.table.get(key)
    }
}
