use crate::net::device;

#[derive(Debug, PartialEq, Eq)]
pub enum PageIndex {
    Welcome,
    Monitor,
    Safe,
    Http,
}
