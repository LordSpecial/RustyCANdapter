mod serial;
mod can_frame;

use std::rc::Rc;
use can_frame::CANFrame;
use slint::{ModelRc, VecModel};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ports_model = Rc::new(VecModel::from(serial::get_serial_ports()));
    ui.set_serial_ports(ports_model.clone().into());

    let table_model = ModelRc::from(Rc::new(VecModel::from(vec![CANFrame::generate_random(Some("100")).to_model_rc(), CANFrame::generate_random(Some("123")).to_model_rc(), CANFrame::generate_random(None).to_model_rc(), CANFrame::generate_random(None).to_model_rc(), CANFrame::generate_random(None).to_model_rc(), CANFrame::generate_random(None).to_model_rc(), CANFrame::generate_random(None).to_model_rc()])));
    ui.set_table_data(table_model);
    ui.run()
}
