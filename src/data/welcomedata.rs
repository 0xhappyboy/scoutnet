use std::sync::Mutex;

use lazy_static::lazy_static;
lazy_static! {
    pub static ref input_text: Mutex<String> = Mutex::new(String::from(""));
}
