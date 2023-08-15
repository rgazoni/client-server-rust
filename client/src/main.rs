use std::net::UdpSocket;
use std::{thread, time};

fn main() {
    // Configuration Settings
    let ip_addr = "127.0.0.1";
    let port_addr = "34254";
    let sleep_duration = 3_000;

    //Program
    let socket = UdpSocket::bind("127.0.0.1:34253").expect("Couldn't bind to address");
    // Pattern is a..z
    let mut pattern: u8 = 'a' as u8;
    loop {
        socket
            .send_to(&[pattern as u8; 1], format!("{}:{}", ip_addr, port_addr))
            .expect("Couldn't send data");
        println!(
            "Message was sent successfully to {}:{} with the message: {}",
            ip_addr, port_addr, pattern as char
        );

        if pattern as char == 'z' {
            pattern = 'a' as u8;
        } else {
            pattern += 1;
        }
        thread::sleep(time::Duration::from_millis(sleep_duration));
    }
}
