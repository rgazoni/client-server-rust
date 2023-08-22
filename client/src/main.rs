use std::net::UdpSocket;
use std::{thread, time};

fn main() {
    // Configuration Settings
    let sleep_duration = 1000;

    for i in 1..=1 {
        thread::spawn(move || {
            let ip_addr = "192.168.1.4";
            let port_addr = "12345";

            let bind_port = 34260 + i;
            let socket =
                UdpSocket::bind(format!("192.168.1.3:1345")).expect("Couldn't bind to address");
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
        });
    }
    loop {}
}
