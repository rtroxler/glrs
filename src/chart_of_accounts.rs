#[derive(Debug, Serialize, Deserialize)]
pub enum AccountCode {
    Base(String),
    Daily(AccrualAccount),
    Periodic(AccrualAccount),
    Cash(CashAccount)
}

#[derive(Debug)]
pub struct DumbAss {
    name: String,
    account_code: AccountCode
}

impl DumbAss {
    fn process(&self) {
        match &self.account_code {
            &AccountCode::Base(ref string) => println!("Can't process AC"),
            &AccountCode::Daily(ref ac) => println!("Daily accrual ac: {:?}", ac),
            &AccountCode::Periodic(ref ac) => println!("Periodic accrual {:?}", ac),
            &AccountCode::Cash(ref ac) => println!("Cash {:?}", ac),
        }
    }
}

#[derive(Debug)]
struct DumbLedger<> {
    asses: Vec<DumbAss>
}
impl DumbLedger {
    fn new() -> DumbLedger {
        let mut l = DumbLedger {
            asses: Vec::new()
        };
        let cash_ab = DumbAss { name: String::from("he"), account_code: AccountCode::Cash(CashAccount { revenue_code: String::from("4000") }) };
        let accrual_ab = DumbAss { name: String::from("he"), account_code: AccountCode::Periodic(AccrualAccount { revenue_code: String::from("4000"), accounts_receivable_code: String::from("1101"), deferred_code: String::from("2020") }) };
        let base_ab = DumbAss { name: String::from("he"), account_code: AccountCode::Base(String::from("1000")) };
        l.asses.push(cash_ab);
        l.asses.push(accrual_ab);
        l.asses.push(base_ab);
        l
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ChartOfAccounts {
    daily_accrual_accounts: Vec<DailyAccrualAccount>,
    accrual_accounts: Vec<AccrualAccount>,
    cash_accounts: Vec<CashAccount>
}

impl ChartOfAccounts {
    pub fn new() -> ChartOfAccounts {
        let mut chart = ChartOfAccounts {
            daily_accrual_accounts: Vec::new(),
            accrual_accounts: Vec::new(),
            cash_accounts: Vec::new(),
        };

        // TODO This should be read in from FMS, but for now let's hard code it to Cubies
        // Basic Rent
        chart.daily_accrual_accounts.push(DailyAccrualAccount {
            revenue_code: String::from("4000"), accounts_receivable_code: String::from("1101"), deferred_code: String::from("2020")
        });
        // Some Fees
        chart.accrual_accounts.push(AccrualAccount {
            revenue_code: String::from("4050"), accounts_receivable_code: String::from("1104"), deferred_code: String::from("")
        });
        chart.accrual_accounts.push(AccrualAccount {
            revenue_code: String::from("4051"), accounts_receivable_code: String::from("1104"), deferred_code: String::from("")
        });
        // Basic Service
        chart.accrual_accounts.push(AccrualAccount {
            revenue_code: String::from("4150"), accounts_receivable_code: String::from("1103"), deferred_code: String::from("2023")
        });
        // Insurance
        chart.cash_accounts.push(CashAccount {
            revenue_code: String::from("4100")
        });
        chart
    }

    pub fn play(&self) {
        let base_ab = DumbAss { name: String::from("he"), account_code: AccountCode::Base(String::from("1000")) };
        println!("base {:?}", base_ab);
        base_ab.process();
        let cash_ab = DumbAss { name: String::from("he"), account_code: AccountCode::Cash(CashAccount { revenue_code: String::from("4000") }) };
        println!("cash {:?}", cash_ab);
        cash_ab.process();
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_types() {
        let mut ledger = DumbLedger::new();
        println!("{:?}", ledger);
        for ass in &ledger.asses {
            ass.process();
        }
    }
}
