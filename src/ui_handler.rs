use slint::{ModelRc, SharedString, StandardListViewItem, VecModel, Weak};
use tokio::sync::mpsc;
use tokio_serial::SerialStream;

use crate::can_frame::{CANFrame, CANFrameStorage};
use std::io::Error as E;
use super::AppWindow;

// Function to handle UI updates
pub async fn ui_update_task(ui_handle: Weak<AppWindow>, mut rx: mpsc::UnboundedReceiver<CANFrame>) {
   println!("Created UI Thread");

    println!("Create Frame Storage and dummy Frames");
    let mut frame_storage = CANFrameStorage::new();
    while let Some(received_frame) = rx.recv().await {
        //println!("Received {}", received_frame.to_string());
        // insert into Hashmap
        frame_storage.update_or_insert(received_frame.clone());

        let storage_clone = frame_storage.clone();
        let ui_handle_clone = ui_handle.clone(); // Clone the handle
        slint::invoke_from_event_loop(move || {
            let models: Vec<ModelRc<StandardListViewItem>> = storage_clone.gather_all_models(); // Replace `SpecificType` with the actual type
            let rc_models: VecModel<ModelRc<StandardListViewItem>> = VecModel::from(models);
            let model_rc: ModelRc<ModelRc<StandardListViewItem>> = ModelRc::new(rc_models);

            if let Some(ui) = ui_handle_clone.upgrade() {
                ui.set_table_data(model_rc);
            } else {
                eprintln!("Could not upgrate ui_handle");
            }
        }).unwrap();
    }
}


pub fn connect_to_com_port(selection: SharedString) -> tokio_serial::Result<SerialStream> {
    let binding = selection.to_string();
    let com_port = binding.split_whitespace().next().unwrap_or_default();
    println!("Connecting");
    let port = SerialStream::open(&tokio_serial::new(com_port, 115200));
    println!("Connectinged");
    return port;
}