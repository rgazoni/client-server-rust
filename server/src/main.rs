use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::net::{SocketAddr, UdpSocket};
use std::time::Instant;

#[derive(Debug)]
struct Connection {
    addr: String,
    current_pattern: u8,
    packet_arrived_counter: u32,
    packet_loss_counter: u32,
    filename: String,
}

impl Connection {
    fn new(addr: String, filename: String, patt: u8) -> Connection {
        Connection {
            addr,
            current_pattern: patt,
            packet_arrived_counter: 1,
            packet_loss_counter: 0,
            filename,
        }
    }
    fn update_pattern(&mut self, patt: u8) -> () {
        if patt.abs_diff(self.current_pattern) == 1
            || (self.current_pattern == 'z' as u8 && patt == 'a' as u8)
        {
            self.current_pattern = patt;
            self.packet_arrived_counter += 1;
        } else {
            self.packet_loss_counter += patt.abs_diff(self.current_pattern - 1) as u32;
            self.current_pattern = patt;
        }
    }
}

fn main() -> std::io::Result<()> {
    {
        let socket: UdpSocket = UdpSocket::bind("127.0.0.1:34254")?;
        let mut conn: HashMap<String, Connection> = HashMap::with_capacity(500);
        let start = Instant::now();

        while start.elapsed().as_secs() < 15 {
            // Caracter per time as the business logic determines
            let mut buf: [u8; 1] = [0; 1];
            let (amt, src): (usize, SocketAddr) = socket.recv_from(&mut buf)?;
            let addr: String = src.to_string();

            // Create a filename
            let mut filename: String = String::from("addr_");
            filename.push_str(&addr);
            filename.push_str(".txt");

            let new_conn: Connection = Connection::new(
                addr.clone(),
                filename.replace(":", "_"),
                *buf.get(0).unwrap(),
            );

            conn.entry(addr.clone()).or_insert(new_conn);

            if let Some(connection) = conn.get_mut(&addr) {
                connection.update_pattern(*buf.get(0).unwrap())
            }

            println!(
                "addr: {} - value: {} hashmap: {:?} - buffer: {:?}",
                src,
                amt,
                &conn.get(&addr).unwrap(),
                &buf
            );

            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(format!("./data/{}", (&conn.get(&addr).unwrap()).filename))?;
            file.write_all(&buf)?;

            let connection = conn.get(&addr).unwrap();
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("./data/performance_log.txt")?;
            file.write_all(
                format!(
                    "IDENTIFIER ADDR: {} - CURRENT_PATTERN: {} PACKET_ARRIVED_COUNTER: {} - PACKET_LOSS_COUNTER: {} \n",
                    connection.addr,
                    connection.current_pattern as char,
                    connection.packet_arrived_counter,
                    connection.packet_loss_counter
                )
                .as_bytes(),
            )?;
        }
        println!("While has been finished!");
        for (key, value) in conn.into_iter() {
            println!("KEY - {}, VALUE {:?}", key, value);
        }
        Ok(())
    }
}
