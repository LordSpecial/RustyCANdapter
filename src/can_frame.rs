use std::collections::HashMap;
use std::rc::Rc;
use slint::{ModelRc, SharedString, StandardListViewItem, VecModel};
use rand::random;

#[derive(Clone)]
pub struct CANFrame {
    pub can_id: String,
    pub byte_1_data: u8,
    pub byte_3_data: u8,
    pub byte_4_data: u8,
    pub byte_2_data: u8,
    pub byte_5_data: u8,
    pub byte_6_data: u8,
    pub byte_7_data: u8,
    pub byte_8_data: u8,
    pub timestamp: u16,
    pub count: u8,
}

pub struct CANFrameStorage {
    frames: HashMap<String, CANFrame>,
}

impl CANFrameStorage {
    pub fn new() -> CANFrameStorage {
        CANFrameStorage {
            frames: HashMap::new(),
        }
    }

    pub fn update_or_insert(&mut self, frame: CANFrame) {
        self.frames.insert(frame.can_id.clone(), frame.clone());
    }

    pub fn get(&self, can_id: String) -> Option<&CANFrame> {
        self.frames.get(&can_id)
    }

    pub fn gather_all_models(&self) -> Vec<ModelRc<StandardListViewItem>> {
        self.frames.values()
            .map(|frame| frame.to_model_rc())
            .collect()
    }
}

impl CANFrame {
    pub fn to_model_rc(&self) -> ModelRc<StandardListViewItem> {
        let vec = vec![
            StandardListViewItem::from(SharedString::from(self.can_id.clone())),
            StandardListViewItem::from(SharedString::from(format!("0x{:02x}", self.byte_1_data))),
            StandardListViewItem::from(SharedString::from(format!("0x{:02x}", self.byte_2_data))),
            StandardListViewItem::from(SharedString::from(format!("0x{:02x}", self.byte_3_data))),
            StandardListViewItem::from(SharedString::from(format!("0x{:02x}", self.byte_4_data))),
            StandardListViewItem::from(SharedString::from(format!("0x{:02x}", self.byte_5_data))),
            StandardListViewItem::from(SharedString::from(format!("0x{:02x}", self.byte_6_data))),
            StandardListViewItem::from(SharedString::from(format!("0x{:02x}", self.byte_7_data))),
            StandardListViewItem::from(SharedString::from(format!("0x{:02x}", self.byte_8_data))),
            StandardListViewItem::from(SharedString::from(self.timestamp.to_string())),
            StandardListViewItem::from(SharedString::from(self.count.to_string())),
        ];
        return ModelRc::from(Rc::new(VecModel::from(vec)));
    }

    pub fn generate_random(can_id: Option<&str>) -> CANFrame {
        CANFrame {
            can_id: can_id // This FUCKING SUCKS but I'm too lazy to figure this out myself
                .map(|s| s.to_string()) // Converts Option<&str> to Option<String>
                .unwrap_or_else(|| {
                    let random_number = random::<u8>(); // Generates a random u8
                    random_number.to_string() // Converts the random number to a String
                }),
            byte_1_data: random::<u8>(),
            byte_3_data: random::<u8>(),
            byte_4_data: random::<u8>(),
            byte_2_data: random::<u8>(),
            byte_5_data: random::<u8>(),
            byte_6_data: random::<u8>(),
            byte_7_data: random::<u8>(),
            byte_8_data: random::<u8>(),
            timestamp: random::<u16>(),
            count: random::<u8>(),
        }
    }
}