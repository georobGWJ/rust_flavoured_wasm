extern crate warsdk;
use warsdk::*;

// Rabbit moves around randomly

#[no_mangle]
pub extern "C" fn botinit() -> i32 {
    loop {
        go(60 + rand(900), 60 + rand(900));
    }
}