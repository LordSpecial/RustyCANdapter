mod serial;

use slint::{SharedString, VecModel};
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ports_model = std::rc::Rc::new(VecModel::from(serial::get_serial_ports()));
    ui.set_serial_ports(ports_model.clone().into());

    ui.run()
}
