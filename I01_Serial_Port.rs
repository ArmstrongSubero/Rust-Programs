use serialport::prelude::*;
use std::time::Duration;
use std::io::Read;
use std::thread;

fn read_from_port(port_name: &str, s: SerialPortSettings) {
    let mut port = match serialport::open_with_settings(&port_name, &s) {
        Ok(port) => port,
        Err(e) => {
            eprintln!("Failed to open port: {}", e);
            return;
        }
    };

    let mut serial_buf: Vec<u8> = vec![0; 1000];  // A buffer to store serial data

    println!("Reading data from {}", port_name);

    loop {
        match port.read(serial_buf.as_mut_slice()) {
            Ok(t) => {
                let s = String::from_utf8_lossy(&serial_buf[..t]);
                print!("{}", s);
            },
            Err(e) => {
                eprintln!("Failed to receive data: {}", e);
                break;
            }
        }
    }
}

fn main() {
    let port_name = "/dev/ttyUSB0";  // Adjust this for your system
    let baud_rate = 9600;

    let s = SerialPortSettings {
        baud_rate,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_millis(10),
    };

    // Spawn a new thread to read from the serial port
    thread::spawn(move || {
        read_from_port(&port_name, s);
    });

    // Your main thread can continue doing other things or just wait
    println!("Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
