#![feature(proc_macro, specialization)]


extern crate pyo3;


use pyo3::prelude::*;


////////////////////////////////////////
// Normal Rust functions
////////////////////////////////////////

fn sum_as_string(a: i64, b: i64) -> String {
    format!("{}", a + b)
}


////////////////////////////////////////
// wrap into CPython
////////////////////////////////////////

#[py::modinit(_foo)]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "sum_as_string")]
    fn sum_as_string_py(a: i64, b: i64) -> PyResult<String> {
       let out = sum_as_string(a, b);
       Ok(out)
    }

    Ok(())
}

// NOTE:
//
// rename the lib_foo.so to _foo.so for CPython after building with Cargo,
// or use "python3 setup.py bdist_wheel" to build Python wheel package directly
//
