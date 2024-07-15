use std::sync::Mutex;

use lazy_static::lazy_static;

use crate::app::config::PageIndex;
lazy_static! {
    pub static ref page_index: Mutex<PageIndex> = Mutex::new(PageIndex::Monitor);
    pub static ref exit: Mutex<bool> = Mutex::new(false);
}
