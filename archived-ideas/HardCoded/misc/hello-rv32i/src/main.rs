#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn _start() {
	main();
}

fn main() {
    let x = "Hello";
    let y = x.as_bytes();
    drop(y);
    // dbg!(z);
    // println!("{x}, world!");
    // asdasdasdasdasdas asdsadas jkash djak dhaksj dhkasjd hasjkdh akjsdh ajksd haskjdh askjd
}
