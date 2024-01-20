mod serial;

use slint::{ModelRc, VecModel, StandardListViewItem};
slint::include_modules!();

fn frame_to_std_list_view_item(){}

fn compile_std_list_view_table() {}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ports_model = std::rc::Rc::new(VecModel::from(serial::get_serial_ports()));
    ui.set_serial_ports(ports_model.clone().into());

    let test: VecModel<StandardListViewItem> =  vec![StandardListViewItem::from("data 1"), StandardListViewItem::from("data 2"), StandardListViewItem::from("data n")].into();
    let test_rc = ModelRc::from(std::rc::Rc::new(VecModel::from(test)));
    let table_model = ModelRc::from(std::rc::Rc::new(VecModel::from(vec![test_rc.clone(), test_rc.clone()])));
    ui.set_table_data_in(table_model);
    ui.run()
}
