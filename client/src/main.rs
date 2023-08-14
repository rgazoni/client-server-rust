use std::net::UdpSocket;
use std::thread;

fn main() {
    let thread_1 = thread::spawn(move || {
        let socket = UdpSocket::bind("127.0.0.1:34253").expect("couldn't bind to address");
        loop {
            socket
                .send_to(&['a' as u8; 1], "127.0.0.1:34254")
                .expect("couldn't send data");
        }
    });

    let thread_2 = thread::spawn(move || {
        let socket = UdpSocket::bind("127.0.0.1:34256").expect("couldn't bind to address");
        loop {
            socket
                .send_to(&['z' as u8; 1], "127.0.0.1:34254")
                .expect("couldn't send data");
        }
    });
    // some work here
    let res1 = thread_1.join();
    let res2 = thread_2.join();
}
