use std::sync::Mutex;

use lazy_static::lazy_static;
lazy_static! {
    pub static ref test_1: Mutex<u64> = { Mutex::new(1) };
    pub static ref test_2: Mutex<u64> = { Mutex::new(2) };
    pub static ref test_3: Mutex<u64> = { Mutex::new(3) };
}
