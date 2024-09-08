fn main() {
    println!("cargo::rustc-check-cfg=cfg(has_foo)");
    //        ^^^^^^^^^^^^^^^^^^^^^^ new with Cargo 1.80
    if has_foo() {
        println!("cargo::rustc-cfg=has_foo");
    }
}


fn has_foo() -> bool {
    true
}