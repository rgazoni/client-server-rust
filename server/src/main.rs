use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::net::{SocketAddr, UdpSocket};

fn main() -> std::io::Result<()> {
    {
        let socket: UdpSocket = UdpSocket::bind("127.0.0.1:34254")?;
        let mut conn: HashMap<String, String> = HashMap::with_capacity(500);

        loop {
            // Caracter per time as the business logic determines
            let mut buf: [u8; 1] = [0; 1];
            let (amt, src): (usize, SocketAddr) = socket.recv_from(&mut buf)?;
            let addr: String = src.to_string();

            // Create a filename
            let mut filename: String = String::from("addr_");
            filename.push_str(&addr);
            filename.push_str(".txt");

            conn.entry(addr.clone()).or_insert(filename);

            println!(
                "addr: {} - value: {} hashmap: {} - buffer: {:?}",
                src,
                amt,
                &conn.get(&addr).unwrap(),
                &buf
            );

            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open((&conn.get(&addr).unwrap()).replace(":", "_"))?;
            file.write_all(&buf)?;
        }
    }
}
