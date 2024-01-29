use std::collections::HashMap;
use std::rc::Rc;
use slint::{ModelRc, SharedString, StandardListViewItem, VecModel};
use rand::random;
use strum_macros::{Display, EnumIter};

#[derive(Copy, Clone, EnumIter, Display, PartialEq)]
pub enum CANRate {
    Kb10 = 0,
    Kb20,
    Kb50,
    Kb100,
    Kb250,
    Kb500,
    Kb800,
    Mb1,
}

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



impl CANFrame {
    pub fn parse(message: String) -> Result<Self, String> {
        // BMS Emulator starts with \r
        if !message.starts_with("\r") {//|| !message.ends_with('\r') {
            return Err("Didn't start or end in T or \\r".to_string());
        }

        let mut chars = message.chars().skip(1); // Skip "b'"

        let is_extended_id = chars.next().ok_or("No next character")? == 'x'; // Check for extended ID
        let id_end = if is_extended_id { 11 } else { 6 };

        let can_id: String = chars.by_ref().take(id_end - 3).collect();

        let data_len = chars.next().ok_or("No next character")?.to_digit(10).ok_or("Not a digit")? as usize;
        if data_len > 8 { return Err("To much data to parse.".to_string()); }

        let data_hex: String = chars.take(data_len * 2).collect();
        let mut data_bytes = Vec::new();
        for i in (0..data_hex.len()).step_by(2) {
            let byte_str = &data_hex[i..i + 2];
            let byte = u8::from_str_radix(byte_str, 16).map_err(|e| e.to_string())?;
            data_bytes.push(byte);
        }

        while data_bytes.len() < 8 {
            data_bytes.push(0);
        }

        Ok(CANFrame {
            can_id,
            byte_1_data: data_bytes[0],
            byte_2_data: data_bytes[1],
            byte_3_data: data_bytes[2],
            byte_4_data: data_bytes[3],
            byte_5_data: data_bytes[4],
            byte_6_data: data_bytes[5],
            byte_7_data: data_bytes[6],
            byte_8_data: data_bytes[7],
            timestamp: 0, // Placeholder
            count: 0,     // Placeholder
        })
    }

    pub fn to_string(&self) -> String {
        let mut msg= String::from(&self.can_id);
        msg.push_str(" ");
        let data = vec![
            self.byte_1_data, self.byte_2_data, self.byte_3_data, self.byte_4_data,
            self.byte_5_data, self.byte_6_data, self.byte_7_data, self.byte_8_data,
        ];
        msg.push_str(&format!("{}", data.iter().filter(|&&byte| byte != 0).count()));
        msg.push_str(" ");
        for &byte in &data {
            msg.push_str(&format!("{:02X},", byte));
        }
        msg.push('\r');
        msg
    }
}

#[derive(Clone)]
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
        let id = can_id // This FUCKING SUCKS, but I'm too lazy to figure this out myself
            .map(|s| s.to_string()) // Converts Option<&str> to Option<String>
            .unwrap_or_else(|| {
                let random_number = random::<u8>(); // Generates a random u8
                random_number.to_string() // Converts the random number to a String
            });

        CANFrame {
            can_id: id,
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