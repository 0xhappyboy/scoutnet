use pnet::datalink::{self, NetworkInterface};

#[derive(Default)]
pub struct Device {}

impl Device {
    pub fn get_device_list() -> Vec<NetworkInterface> {
        datalink::interfaces()
    }
    pub fn get_device_name_list() -> Vec<String> {
        let device_list = datalink::interfaces();
        let mut r_vec = Vec::<String>::default();
        for (index, v) in device_list.iter().enumerate() {
            r_vec.push(v.name.clone());
        }
        r_vec
    }
}
