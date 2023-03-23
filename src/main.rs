use std::net::UdpSocket;

use dns_server::{BytePacketBuffer, DnsPacket, DnsQuestion, QueryType, Result};
use rand;
fn main() -> Result<()> {
    // c2::main();
    // return Ok(());

    let qname = "www.yahoo.com";
    let qtype = QueryType::A;

    let server = ("127.0.0.1", 53);

    let socket = UdpSocket::bind(("0.0.0.0", 43210))?;

    let mut packet = DnsPacket::new();

    let n: u16 = rand::random();

    packet.header.id = n;
    packet.header.questions = 1;

    packet.header.recursion_desired = true;
    packet
        .questions
        .push(DnsQuestion::new(qname.to_string(), qtype));

    let mut req_buffer = BytePacketBuffer::new();
    packet.write(&mut req_buffer)?;

    socket.send_to(&req_buffer.buf[0..req_buffer.pos], server)?;

    let mut res_buffer = BytePacketBuffer::new();
    println!("Awaiting response");
    socket.recv_from(&mut res_buffer.buf)?;

    let res_packet = DnsPacket::from_buffer(&mut res_buffer)?;
    println!("{:#?}", res_packet.header);

    for q in res_packet.questions {
        println!("{:#?}", q);
    }
    for rec in res_packet.answers {
        println!("{:#?}", rec);
    }
    for rec in res_packet.authorities {
        println!("{:#?}", rec);
    }
    for rec in res_packet.resources {
        println!("{:#?}", rec);
    }

    Ok(())
}
