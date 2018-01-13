extern {
    fn func();
}

fn main() {
    unsafe {
        func();
    }
}
