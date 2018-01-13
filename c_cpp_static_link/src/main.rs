extern {
    fn func0();
    fn func1();
    fn func2();
    fn func3();
}

fn main() {
    unsafe {
        func0();
        func1();
        func2();
        func3();
    }
}
