extern crate libc;
extern crate rustc_serialize;
use rustc_serialize::json;

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

    println!("Rust input argument: {:?}", input);

    // TODO: let us return GLs (chrono serialization)
    let general_ledger = ledger.process_general_ledger();

    let result = OutputArg { some_integer: 42, some_string: "the quick brown fox".to_string(), another_string: "jumps over the lazy dog".to_string() };

    //println!("Rust result: {:?}", result);

    let string_result = result.to_json();
    conversions::c_ptr_from_string(&string_result)
}

#[no_mangle]
pub extern "C" fn rust_free(c_ptr: *mut libc::c_void) {
    unsafe {
        libc::free(c_ptr);
    }
}


#[derive(Debug, RustcDecodable)]
struct InputArgs {
    ledger: ledger::Ledger
}

impl InputArgs {
    pub fn from_json(json_string: &str) -> InputArgs {
        json::decode(json_string).unwrap()
    }
}

// This will probably be a GeneralLedger
#[derive(Debug, RustcEncodable)]
struct OutputArg {
    some_integer: i32,
    some_string: String,
    another_string: String,
}

impl  OutputArg {
    pub fn to_json(&self) -> String {
        json::encode(self).unwrap()
    }
}
