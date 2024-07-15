use std::sync::Mutex;

use lazy_static::lazy_static;
lazy_static! {
    pub static ref exit: Mutex<bool> = { Mutex::new(false) };
}
