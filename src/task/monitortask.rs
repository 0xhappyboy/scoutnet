use std::{thread, time::Duration};

use crate::{data::appdata::exit, net::pack::NetPack};

pub async fn get_real_time_net_pack() {
    NetPack::listen("en0".to_string());
}
