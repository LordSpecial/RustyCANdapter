use std::rc::Rc;

use slint::{SharedString, VecModel};
use tokio::runtime::Builder;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use can_frame::CANFrame;

mod serial;
mod can_frame;
mod ui_handler;

slint::include_modules!();

fn main() -> () {
    println!("Create App & Channels");
    let ui = AppWindow::new().unwrap();
    let ui_handle = ui.as_weak();  // Create a weak handle to access the UI from the async task
    let (tx, rx): (UnboundedSender<CANFrame>, UnboundedReceiver<CANFrame>) = mpsc::unbounded_channel();

    println!("Create Ports Model");
    let ports_model = Rc::new(VecModel::from(serial::get_serial_ports()));

    println!("Insert Ports Model");
    ui.set_serial_ports(ports_model.clone().into());

// Set up the Tokio runtime
    println!("Creat Tokio Runtime");
    let rt = Builder::new_multi_thread().enable_all().build().unwrap();

    // Connect to serial Port
    ui.on_fuckoff(move |string| {
        ui_handler::connect_to_com_port(string);
    });

    println!("Start Serial Monitor");
    // rt.spawn(serial::serial_port_monitor, tx));
    println!("Start UI handler");
    rt.spawn(ui_handler::ui_update_task(ui_handle, rx));
    // Keep the main thread running
    rt.block_on(async {
        // Perform other async tasks or just keep the thread alive
    });
    println!("RUN");
   let _ = ui.run();

}
