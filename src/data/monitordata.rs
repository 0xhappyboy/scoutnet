use std::{collections::HashMap, sync::Mutex};

use lazy_static::lazy_static;
use pnet::datalink::NetworkInterface;
use ratatui::widgets::{ScrollbarState, TableState};
use tui_tree_widget::{TreeItem, TreeState};

use crate::app::ui::{
    config::MonitorPageArea,
    monitor::{DEVICE_TABLE_ITEM_HEIGHT, REAL_TIME_NET_PACK_TABLE_ITEM_HEIGHT},
};
lazy_static! {
    pub static ref test_1: Mutex<u64> = Mutex::new(1);
    pub static ref test_2: Mutex<u64> = Mutex::new(2);
    pub static ref test_3: Mutex<u64> = Mutex::new(3);
    pub static ref monitor_page_selected_area: Mutex<MonitorPageArea> =
        Mutex::new(MonitorPageArea::None);
    pub static ref monitor_page_selecting_area: Mutex<MonitorPageArea> =
        Mutex::new(MonitorPageArea::Area_1);
    pub static ref monitor_page_device_table_state: Mutex<TableState> =
        Mutex::new(TableState::default());
    pub static ref monitor_page_net_pack_info_tree_items: Mutex<Vec<TreeItem<'static, &'static str>>> = {
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
    pub static ref monitor_page_net_pack_info_tree_state: Mutex<TreeState<&'static str>> =
        Mutex::new(TreeState::default());
    pub static ref monitor_page_device_table_data: Mutex<Vec<NetworkInterface>> =
        Mutex::new(vec![]);
    pub static ref monitor_page_device_table_selected_index: Mutex<Option<usize>> =
        Mutex::new(Some(0));
    pub static ref monitor_page_real_time_net_pack_table_selected_index: Mutex<Option<usize>> =
        Mutex::new(Some(0));
    pub static ref monitor_page_real_time_net_pack_table_state: Mutex<TableState> =
        Mutex::new(TableState::default());
    pub static ref monitor_page_device_table_scroll_bar_state: Mutex<ScrollbarState> = {
        Mutex::new(ScrollbarState::new(
            (monitor_page_device_table_data.lock().unwrap().len() - 1) * DEVICE_TABLE_ITEM_HEIGHT,
        ))
    };
    pub static ref monitor_page_real_time_net_pack_table_scroll_bar_state: Mutex<ScrollbarState> = {
        Mutex::new(ScrollbarState::new(
            (monitor_page_real_time_net_pack_table_data
                .lock()
                .unwrap()
                .len()
                - 1)
                * REAL_TIME_NET_PACK_TABLE_ITEM_HEIGHT,
        ))
    };
    pub static ref monitor_page_real_time_net_pack_table_data: Mutex<Vec<HashMap<String, String>>> = {
        let mut m = HashMap::new();
        m.insert(String::from("k1"), String::from("v1"));
        m.insert(String::from("k2"), String::from("v2"));
        m.insert(String::from("k3"), String::from("v3"));
        m.insert(String::from("k4"), String::from("v3"));
        m.insert(String::from("k5"), String::from("v3"));
        m.insert(String::from("k6"), String::from("v3"));
        m.insert(String::from("k7"), String::from("v3"));

        let mut m2 = HashMap::new();
        m2.insert(String::from("k1"), String::from("vvvvvv1"));
        m2.insert(String::from("k2"), String::from("vvvvvv2"));
        m2.insert(String::from("k3"), String::from("vvvvvv3"));
        m2.insert(String::from("k4"), String::from("vvvvvv4"));
        m2.insert(String::from("k5"), String::from("vvvvvv5"));
        m2.insert(String::from("k6"), String::from("vvvvvqwerqwvv6"));
        m2.insert(
            String::from("k7"),
            String::from("vvvqvqwerqwwerqwerqwervvv7"),
        );
        m2.insert(String::from("k8"), String::from("vvvvvvqwerqwvqwvvvvvvqwerqwvqwerqwvqwerqwvqwerqwv8vvvvvvqwerqwvqwerqwvqwerqwvqwerqwv8vvvvvvqwerqwvqwerqwvqwerqwvqwerqwv8erqwvqwerqwvqwerqwv8"));

        let t_data: Vec<HashMap<String, String>> = vec![m.clone(), m2.clone()];
        Mutex::new(t_data)
    };
}
