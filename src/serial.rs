use slint::SharedString;

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