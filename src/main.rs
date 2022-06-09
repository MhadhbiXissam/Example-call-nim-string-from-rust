extern crate libc;

use libc::c_char;
use std::ffi::CStr;
use std::str;



extern "C"  {
    fn NimMain();
    fn hello() -> *const c_char;
}

fn main() {
    let c_buf: *const c_char = unsafe { hello() };
    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
    let str_slice: &str = c_str.to_str().unwrap();
    let str_buf: String = str_slice.to_owned();  // if necessary
    println!("string from nim  = fibs() = {}" ,  str_buf);
}