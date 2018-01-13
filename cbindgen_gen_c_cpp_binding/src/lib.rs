use std::ffi::CString;
use std::os::raw::c_char;


#[no_mangle]
pub extern "C" fn func0() {
    println!("Hello, world!");
}

#[no_mangle]
pub extern "C" fn func1(a: u32, b: u32) -> u32 {
    a * b
}

#[no_mangle]
pub extern "C" fn func2(n: usize) -> Vec<i32> {
    vec![10; n]
}

#[derive(Debug)]
pub enum Foo{
    A,
    B,
}

#[no_mangle]
pub extern "C" fn func3(data: Foo) -> String {
    format!("{:?}", data)
}

#[no_mangle]
pub extern "C" fn func4(data: Foo) -> *const c_char {
    CString::new(format!("{:?}", data)).unwrap().as_ptr()
}
