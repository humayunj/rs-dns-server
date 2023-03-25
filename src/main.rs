use std::net::UdpSocket;

use dns_server::{handle_query, Result};
use rand;
fn main() -> Result<()> {
    // c2::main();
    // return Ok(());
    let port = 2053;
    println!("DNS Server listening on port: {}", port);
    let socket = UdpSocket::bind(("0.0.0.0", port))?;

    loop {
        match handle_query(&socket) {
            Ok(_) => {}
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
