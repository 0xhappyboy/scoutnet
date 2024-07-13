pub const TABS: [&str; 4] = ["Home", "Monitor", "Safe", "Http"];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MonitorPageArea {
    Area_1,
    Area_2,
    Area_3,
    Area_4,
    None,
}
