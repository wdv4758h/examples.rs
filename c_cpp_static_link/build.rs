extern crate cc;


fn main() {
    // env like CC, CFLAGS, CXX, CXXFLASG, AR work

    // optimization flags will choose automatically,
    // unless you want to use different flags

    // C - Basic
    cc::Build::new()
        .file("c_cpp_src/func0.c")
        .compile("myfunc0");

    // C++ - Basic
    cc::Build::new()
        .cpp(true)
        .file("c_cpp_src/func1.cpp")
        .compile("myfunc1");

    // C - Advance
    cc::Build::new()
        .warnings(true)
        .warnings_into_errors(true)
        .file("c_cpp_src/func2.c")
        .compile("myfunc2");

    // C++ - Advance
    cc::Build::new()
        .cpp(true)
        .warnings(true)
        .warnings_into_errors(true)
        .file("c_cpp_src/func3.cpp")
        .compile("myfunc3");
}
