extern crate libc;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate chrono;

mod ledger;
mod usd;
mod account_map;

mod conversions;

#[no_mangle]
pub extern "C" fn rust_perform(c_ptr: *const libc::c_char) -> *const libc::c_char {
    let string_arg = conversions::string_from_c_ptr(c_ptr);

    let input = InputArgs::from_json(&string_arg);
    let ledger = &input.ledger;

    // TODO: let us return GLs (chrono serialization)
    let general_ledger = ledger.process_general_ledger();
    //general_ledger.print();

    let result = OutputArg { some_integer: 42, some_string: "the quick brown fox".to_string(), another_string: "jumps over the lazy dog".to_string() };

    println!("right before it");
    let string_result = general_ledger.to_json();
    println!("Rust result: {:?}", string_result);
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

// This will probably be a GeneralLedger
#[derive(Debug, Serialize, Deserialize)]
struct OutputArg {
    some_integer: i32,
    some_string: String,
    another_string: String,
}
