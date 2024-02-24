use std::time::Duration;
use slint::SharedString;
use tokio::io;

use tokio::sync::mpsc;
use tokio_serial::SerialStream;
use crate::can_frame::CANFrame;

pub fn get_serial_ports() -> Vec<SharedString> {
    let ports: Vec<SharedString> = tokio_serial::available_ports()
        .unwrap()
        .iter()
        .map(|port| {
            let port_type_description = match &port.port_type {
                tokio_serial::SerialPortType::UsbPort(info)
                => format!("USB Port - VID: {}, PID: {}", info.vid, info.pid),
                // Add other matches for different port types as needed
                _ => "Unknown".to_string(), // Fallback for any unhandled types
            };
            SharedString::from(format!("{} ({})", port.port_name, port_type_description))
        })
        .collect();
    return ports;
}

pub async fn serial_port_monitor(mut port: SerialStream, tx: mpsc::UnboundedSender<CANFrame>) {
    println!("Created Serial Thread");


    println!("Success");
    let mut buf = vec![0; 1024];
    loop {
        // Would do readable but it shits the bed
        match port.try_read(&mut buf) {
            Ok(n) if n > 0 => {
                let received_data = String::from_utf8_lossy(&buf[..n]);
                //println!("Serial Received: {}", received_data.to_string());
                let _ = match CANFrame::parse(received_data.to_string()) {
                    Ok(frame) => {
                        //println!("Sending  {}", frame.to_string());
                        tx.send(frame)
                    },
                    Err(e) => {
                        println!("Error parsing CAN frame: {:?}", e);
                        continue;
                    },
                };

            },
            Ok(_) => {},
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {},
            Err(e) => {
                eprintln!("Error reading from serial port: {:?}", e);
                break;
            }
        };
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}