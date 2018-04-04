extern crate libc;
extern crate rustc_serialize;
use rustc_serialize::json;

extern crate chrono;
use chrono::prelude::*;

mod account_map;
mod transaction;

use transaction::*;

mod usd;
use usd::USD;

mod general_ledger;
use general_ledger::GeneralLedger;

#[no_mangle]
pub extern "C" fn rust_perform(c_ptr: *const libc::c_char) -> *const libc::c_char {
    let string_arg = string_from_c_ptr(c_ptr);
    println!("{:?}", string_arg);

    let arg = InputArg::from_json(&string_arg);

    println!("Rust input argument: {:?}", arg);

    let result = OutputArg { some_integer: 42, some_string: "the quick brown fox".to_string(), another_string: "jumps over the lazy dog".to_string() };
    println!("Testing output ... ");

    println!("Rust result: {:?}", result);

    let string_result = result.to_json();
    c_ptr_from_string(&string_result)
}

#[no_mangle]
pub extern "C" fn rust_free(c_ptr: *mut libc::c_void) {
    unsafe {
        libc::free(c_ptr);
    }
}

fn string_from_c_ptr(c_ptr: *const libc::c_char) -> String {
    let c_str = unsafe {
        assert!(!c_ptr.is_null());
        std::ffi::CStr::from_ptr(c_ptr)
    };
    c_str.to_str().unwrap().to_string()
}

fn c_ptr_from_string(s: &str) -> *const libc::c_char {
    std::ffi::CString::new(s).unwrap().into_raw()
}

// How do I handle mixed inputs? ABs and ABIs?
// Replace this with Transaction::Assessments, etc?
#[derive(Debug, RustcDecodable)]
struct InputArg {
    some_integer: i32,
    some_string: String,
    many_strings: Vec<String>
}

impl InputArg {
    pub fn from_json(json_string: &str) -> InputArg {
        json::decode(json_string).unwrap()
    }
}

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
