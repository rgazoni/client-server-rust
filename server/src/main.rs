use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("127.0.0.1:34254")?;
        let mut conn: HashMap<String, String> = HashMap::with_capacity(1000);

        loop {
            // Caracter per time as the business logic determines
            let mut buf = [0; 1];
            let (amt, src) = socket.recv_from(&mut buf)?;
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
                .open(&conn.get(&addr).unwrap())?;
            // let mut file = File::create(&conn.get(&addr).unwrap())?;
            file.write_all(&buf)?;
        }
    }
    Ok(())
}
