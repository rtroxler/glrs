#[derive(Debug)]
pub struct DumbAss<AC> {
    name: String,
    account_code: AC
}

impl<AC> DumbAss<AC> {
    fn ac(&self) -> &AC {
        &self.account_code
    }
}

impl DumbAss<String> {
    fn process(&self) {
        println!("Can't process")
    }
}
impl DumbAss<CashAccount> {
    fn process(&self) {
        println!("PROCESSING CASH!")
    }
}
impl DumbAss<DailyAccrualAccount> {
    fn process(&self) {
        println!("PROCESSING DAILY!")
    }
}
impl DumbAss<AccrualAccount> {
    fn process(&self) {
        println!("PROCESSING ACCRUAL!")
    }
}

#[derive(Debug)]
pub enum AccountCode<AC, S> {
    Mapped(AC),
    Base(S)
}

trait TypedAccountCode {} // not needed?

#[derive(Debug, Serialize, Deserialize)]
struct DailyAccrualAccount {
    account_code: String,
    accounts_receivable_code: String,
    deferred_code: String
}
impl TypedAccountCode for DailyAccrualAccount {}

#[derive(Debug, Serialize, Deserialize)]
struct AccrualAccount {
    account_code: String,
    accounts_receivable_code: String,
    deferred_code: String
}
impl TypedAccountCode for AccrualAccount {}

#[derive(Debug, Serialize, Deserialize)]
struct CashAccount {
    account_code: String
}
impl TypedAccountCode for CashAccount {}

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
            account_code: String::from("4000"), accounts_receivable_code: String::from("1101"), deferred_code: String::from("2020")
        });
        // Some Fees
        chart.accrual_accounts.push(AccrualAccount {
            account_code: String::from("4050"), accounts_receivable_code: String::from("1104"), deferred_code: String::from("")
        });
        chart.accrual_accounts.push(AccrualAccount {
            account_code: String::from("4051"), accounts_receivable_code: String::from("1104"), deferred_code: String::from("")
        });
        // Basic Service
        chart.accrual_accounts.push(AccrualAccount {
            account_code: String::from("4150"), accounts_receivable_code: String::from("1103"), deferred_code: String::from("2023")
        });
        // Insurance
        chart.cash_accounts.push(CashAccount {
            account_code: String::from("4100")
        });
        chart
    }

    pub fn play(&self) {
        let base_ab = DumbAss::<String> { name: String::from("he"), account_code: String::from("1000") };
        println!("base {:?}", base_ab);
        base_ab.process();
        let cash_ab = DumbAss { name: String::from("he"), account_code: CashAccount { account_code: String::from("4000") } };
        println!("cash {:?}", cash_ab);
        cash_ab.process();
    }
}
