#![no_std]
#![no_main]
// use libc::{c_int};
// gpt does NOT recommend this band-aid
#[cfg(not(test))]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[no_mangle]
pub extern "C" fn start() -> () {
    main();
    // loop {} // infinite loop to prevent exit
}

fn main(){
    ()
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}