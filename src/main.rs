mod serial;
mod can_frame;

use std::rc::Rc;
use can_frame::CANFrame;
use slint::{ModelRc, VecModel};
use crate::can_frame::CANFrameStorage;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let mut frame_storage = CANFrameStorage::new();

    // Fake Frames
    frame_storage.update_or_insert(CANFrame::generate_random(Some("100")));
    frame_storage.update_or_insert(CANFrame::generate_random(Some("123")));
    frame_storage.update_or_insert(CANFrame::generate_random(None));
    frame_storage.update_or_insert(CANFrame::generate_random(None));
    frame_storage.update_or_insert(CANFrame::generate_random(None));
    frame_storage.update_or_insert(CANFrame::generate_random(None));
    frame_storage.update_or_insert(CANFrame::generate_random(None));

    let ports_model = Rc::new(VecModel::from(serial::get_serial_ports()));
    ui.set_serial_ports(ports_model.clone().into());

    let table_model = ModelRc::from(Rc::new(VecModel::from(frame_storage.gather_all_models())));
    ui.set_table_data(table_model);
    ui.run()
}
