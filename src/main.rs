#[macro_use]
extern crate lazy_static;

use std::os::raw::c_char;
use std::ffi::CStr;
use std::str;
use std::fmt::Write;
use std::sync::Mutex;

lazy_static! {
    static ref HEAP_STRING: Mutex<String> = Mutex::new(String::new());
}

#[no_mangle]
pub extern "C" fn wasm_number(n: i32, m: f64) -> f64 {
    m + (n as f64) * 2.0
}

#[no_mangle]
pub extern "C" fn wasm_string_to_rust(c_buf: *const c_char) {
    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
    let buffer: &[u8] = c_str.to_bytes();
    let rust_str: &str = str::from_utf8(buffer).unwrap();
    let rust_string: String = rust_str.to_owned();
    println!("{}", rust_string);
}

#[no_mangle]
pub extern "C" fn wasm_string_to_js() -> *const c_char {
    let s: String = "string from rust".to_string();

    let mut output = HEAP_STRING.lock().unwrap();
    output.clear();
    write!(output, "{}\0", s).unwrap();
    output.as_ptr() as *const c_char
}

#[no_mangle]
pub extern "C" fn wasm_array_to_js(_len: i32, ptr: *mut f32) {
    let len = _len as usize;
    let mut v: Vec<f32> = vec![0.5; len];
    v[3] = 99.3;
    let mut res: &mut [f32] = unsafe { std::slice::from_raw_parts_mut(ptr, len) };
    res.clone_from_slice(&v.as_slice())
}

#[no_mangle]
pub extern "C" fn wasm_array_to_rust(_len: i32, ptr: *const f32) {
    let len = _len as usize;
    let slice: &[f32] = unsafe { std::slice::from_raw_parts(ptr, len) };
    for n in slice {
        println!("{}", n);
    }
}

fn main() {}
