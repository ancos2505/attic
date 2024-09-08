fn main() {
    println!("Hello, world! [{:?}]", get_foo());
}

fn get_foo() -> Option<u8> {
    if cfg!(has_foo) {
        Some(42)
    } else {
        None
    }
}
