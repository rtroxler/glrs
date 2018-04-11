// This should be replaced by a legit AccountCode type set
pub fn accounts_receivable_code(revenue_code: &str) -> String {
    match revenue_code {
        "4000" => String::from("1101"),
        "4050" => String::from("1103"), // invoice fee? idk, but 1103 is A/R Service
        "4051" => String::from("1104"), // This is gonna be cumbersome unless I set up a better mapping system
        "4150" => String::from("1103"),
        _ => String::from("FUCK")
    }
}

pub fn deferred_code(revenue_code: &str) -> String {
    match revenue_code {
        "4000" => String::from("2020"),
        "4150" => String::from("2023"),
        _ => String::from("FUCK")
    }
}
