use slint::{ModelRc, StandardListViewItem, VecModel, Weak};
use tokio::sync::mpsc;

use crate::can_frame::{CANFrame, CANFrameStorage};

use super::AppWindow;

// Function to handle UI updates
pub async fn ui_update_task(ui_handle: Weak<AppWindow>, mut rx: mpsc::UnboundedReceiver<CANFrame>) {
   println!("Created UI Thread");

    println!("Create Frame Storeage and dummy Frames");
    let mut frame_storage = CANFrameStorage::new();
    while let Some(received_frame) = rx.recv().await {
        println!("Received {}", received_frame.to_string());
        // insert into Hashmap
        println!("Store Frame");
        frame_storage.update_or_insert(received_frame.clone());
        println!("Successfully Stored Frame {} in Hash",frame_storage.get(received_frame.can_id.clone()).unwrap().to_string());

        let storage_clone = frame_storage.clone();
        println!("Found {} in Cloned Hash",storage_clone.get(received_frame.can_id.clone()).unwrap().to_string());
        let ui_handle_clone = ui_handle.clone(); // Clone the handle
        slint::invoke_from_event_loop(move || {
            println!("Found {} in invoked Cloned Hash",storage_clone.get(received_frame.can_id.clone()).unwrap().to_string());
            let models: Vec<ModelRc<StandardListViewItem>> = storage_clone.gather_all_models(); // Replace `SpecificType` with the actual type
            let rc_models: VecModel<ModelRc<StandardListViewItem>> = VecModel::from(models);
            let model_rc: ModelRc<ModelRc<StandardListViewItem>> = ModelRc::new(rc_models);

            if let Some(ui) = ui_handle_clone.upgrade() {
                println!("actually write the data");
                ui.set_table_data(model_rc);
                // ui.set_test("fuck off".into());
                println!("Wrote the data");
            } else {
                println!("well fuck")
            }
        }).unwrap();
    }
}
