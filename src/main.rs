use slint::{SharedString, VecModel};
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ports: Vec<SharedString> = vec!["Example 1", "String 2", "String 3"]
        .into_iter()
        .map(|s| SharedString::from(s))
        .collect();
    let ports_model = std::rc::Rc::new(VecModel::from(ports));
    ui.set_serial_ports(ports_model.clone().into());

    ui.run()
}
