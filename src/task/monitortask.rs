use std::{thread, time::Duration};

use crate::data::{
    appdata::exit,
    monitordata::{test_1, test_2, test_3},
};

pub async fn get_real_time_net_pack() {}

pub async fn test1() {
    loop {
        thread::sleep(Duration::from_millis(1000));
        if (*test_1.lock().unwrap() > 10) {
            *exit.lock().unwrap() = true;
        } else {
            *test_1.lock().unwrap() += 1;
        }
    }
}
pub async fn test2() {
    loop {
        thread::sleep(Duration::from_millis(1000));
        *test_2.lock().unwrap() += 1;
    }
}
pub async fn test3() {
    loop {
        thread::sleep(Duration::from_millis(1000));
        *test_3.lock().unwrap() += 1;
    }
}
