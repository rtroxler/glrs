// TODO: Remove me
#![allow(dead_code)]

extern crate libc;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate chrono;

mod ledger;
use ledger::general_ledger::GeneralLedger;
mod usd;
mod account_map;
mod chart_of_accounts;

mod conversions;

#[no_mangle]
pub extern "C" fn rust_perform(c_ptr: *const libc::c_char) -> *const libc::c_char {
    let string_arg = conversions::string_from_c_ptr(c_ptr);

    let input = InputArgs::from_json(&string_arg);
    let ledger = &input.ledger;

    let result = OutputArg {
        general_ledger: ledger.process_general_ledger()
    };

    let string_result = result.to_json();
    conversions::c_ptr_from_string(&string_result)
}

#[no_mangle]
pub extern "C" fn rust_free(c_ptr: *mut libc::c_void) {
    unsafe {
        libc::free(c_ptr);
    }
}


#[derive(Debug, Serialize, Deserialize)]
struct InputArgs {
    ledger: ledger::Ledger
}

impl InputArgs {
    pub fn from_json(json_string: &str) -> InputArgs {
        serde_json::from_str(json_string).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct OutputArg {
    general_ledger: GeneralLedger
}

impl  OutputArg {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
