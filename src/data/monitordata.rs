use std::{collections::HashMap, sync::Mutex};

use lazy_static::lazy_static;
use pnet::datalink::NetworkInterface;
use ratatui::widgets::{ScrollbarState, TableState};
use tui_tree_widget::{TreeItem, TreeState};

use crate::{
    app::ui::{
        config::MonitorPageArea,
        monitor::{DEVICE_TABLE_ITEM_HEIGHT, REAL_TIME_NET_PACK_TABLE_ITEM_HEIGHT},
    },
    net::pack::NetPack,
};
lazy_static! {
    // selected area
    pub static ref selected_area: Mutex<MonitorPageArea> = Mutex::new(MonitorPageArea::None);
    // selecting area
    pub static ref selecting_area: Mutex<MonitorPageArea> = Mutex::new(MonitorPageArea::Area_1);
    // device table state
    pub static ref device_table_state: Mutex<TableState> = Mutex::new(TableState::default());
    // current device name
    pub static ref current_device_name: Mutex<String> =  Mutex::new(String::default());
    // net pack info tree items
    pub static ref net_pack_info_tree_items: Mutex<Vec<TreeItem<'static, &'static str>>> = {
        Mutex::new(vec![
            TreeItem::new_leaf("a", "Alfa"),
            TreeItem::new(
                "b",
                "Bravo",
                vec![
                    TreeItem::new_leaf("c", "Charlie"),
                    TreeItem::new(
                        "d",
                        "Delta",
                        vec![
                            TreeItem::new_leaf("e", "Echo"),
                            TreeItem::new_leaf("f", "Foxtrot"),
                        ],
                    )
                    .expect("all item identifiers are unique"),
                    TreeItem::new_leaf("g", "Golf"),
                ],
            )
            .expect("all item identifiers are unique"),
            TreeItem::new_leaf("h", "Hotel"),
            TreeItem::new(
                "i",
                "India",
                vec![
                    TreeItem::new_leaf("j", "Juliett"),
                    TreeItem::new_leaf("k", "Kilo"),
                    TreeItem::new_leaf("l", "Lima"),
                    TreeItem::new_leaf("m", "Mike"),
                    TreeItem::new_leaf("n", "November"),
                ],
            )
            .expect("all item identifiers are unique"),
            TreeItem::new_leaf("o", "Oscar"),
            TreeItem::new(
                "p",
                "Papa",
                vec![
                    TreeItem::new_leaf("q", "Quebec"),
                    TreeItem::new_leaf("r", "Romeo"),
                    TreeItem::new_leaf("s", "Sierra"),
                    TreeItem::new_leaf("t", "Tango"),
                    TreeItem::new_leaf("u", "Uniform"),
                    TreeItem::new(
                        "v",
                        "Victor",
                        vec![
                            TreeItem::new_leaf("w", "Whiskey"),
                            TreeItem::new_leaf("x", "Xray"),
                            TreeItem::new_leaf("y", "Yankee"),
                        ],
                    )
                    .expect("all item identifiers are unique"),
                ],
            )
            .expect("all item identifiers are unique"),
            TreeItem::new_leaf("z", "Zulu"),
        ])
    };
    // net pack info tree state
    pub static ref net_pack_info_tree_state: Mutex<TreeState<&'static str>> = Mutex::new(TreeState::default());
    // device table data
    pub static ref device_table_data: Mutex<Vec<NetworkInterface>> = Mutex::new(vec![]);
    pub static ref device_table_selected_index: Mutex<Option<usize>> = Mutex::new(Some(0));
    pub static ref real_time_net_pack_table_selected_index: Mutex<Option<usize>> = Mutex::new(Some(0));
    pub static ref real_time_net_pack_table_state: Mutex<TableState> = Mutex::new(TableState::default());
    pub static ref device_table_scroll_bar_state: Mutex<ScrollbarState> = {
        Mutex::new(ScrollbarState::new((device_table_data.lock().unwrap().len() - 1) * DEVICE_TABLE_ITEM_HEIGHT,))
    };
    // real time net table scroll bar state
    pub static ref real_time_net_pack_table_scroll_bar_state: Mutex<ScrollbarState> = {
        Mutex::new(ScrollbarState::new((real_time_net_pack_table_data.lock().unwrap().len() - 1) * REAL_TIME_NET_PACK_TABLE_ITEM_HEIGHT,))
    };
    pub static ref real_time_net_pack_table_data: Mutex<Vec<NetPack>> = {
        Mutex::new(vec![NetPack::default()])
    };
    pub static ref real_time_net_pack_table_data_no: Mutex<u128> = Mutex::new(1);
}
