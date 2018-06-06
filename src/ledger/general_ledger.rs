extern crate chrono;
use chrono::prelude::*;
use std::collections::BTreeMap;
use std::collections::HashMap;

extern crate serde;
extern crate serde_json;

use usd::USD;

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneralLedger {
    // By Day
    #[serde(with = "map_as_pairs")]
    entries: HashMap<(NaiveDate, String), USD>,
}

impl GeneralLedger {
    pub fn new() -> GeneralLedger {
        GeneralLedger {
            entries: HashMap::new(),
        }
    }

    pub fn print(&self) {
        // Debug purposes
        println!("|    Date    | Acct | Debit | Credit |");
        println!("--------------------------------------");
        let ordered: BTreeMap<_, _> = self.entries.iter().collect();
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

    pub fn record_double_entry(
        &mut self,
        date: NaiveDate,
        amount: USD,
        debit_account_code: &String,
        credit_account_code: &String,
    ) {
        {
            let debit = self.entries
                .entry((date, debit_account_code.clone()))
                .or_insert(USD::zero());
            *debit += amount;
        }
        {
            let credit = self.entries
                .entry((date, credit_account_code.clone()))
                .or_insert(USD::zero());
            *credit -= amount;
        }
    }

    pub fn fetch_amount(&self, date: NaiveDate, account_code: String) -> Option<&USD> {
        self.entries.get(&(date, account_code))
    }

    pub fn entries(&self) -> &HashMap<(NaiveDate, String), USD> {
        &self.entries
    }
}

// for serializing the tuple key of GL#entries
mod map_as_pairs {
    use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};
    use serde::ser::{Serialize, Serializer};
    use std::fmt;
    use std::marker::PhantomData;

    pub fn serialize<K, V, M, S>(map: M, serializer: S) -> Result<S::Ok, S::Error>
    where
        K: Serialize,
        V: Serialize,
        M: IntoIterator<Item = (K, V)>,
        S: Serializer,
    {
        serializer.collect_seq(map)
    }

    pub fn deserialize<'de, K, V, M, D>(deserializer: D) -> Result<M, D::Error>
    where
        K: Deserialize<'de>,
        V: Deserialize<'de>,
        M: Default + Extend<(K, V)>,
        D: Deserializer<'de>,
    {
        struct MapVisitor<K, V, M> {
            keys: PhantomData<K>,
            values: PhantomData<V>,
            map: PhantomData<M>,
        }

        impl<'de, K, V, M> Visitor<'de> for MapVisitor<K, V, M>
        where
            K: Deserialize<'de>,
            V: Deserialize<'de>,
            M: Default + Extend<(K, V)>,
        {
            type Value = M;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a sequence of key-value pairs")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut map = M::default();
                while let Some((k, v)) = seq.next_element()? {
                    map.extend(Some((k, v)));
                }
                Ok(map)
            }
        }

        deserializer.deserialize_seq(MapVisitor {
            keys: PhantomData,
            values: PhantomData,
            map: PhantomData,
        })
    }
}
