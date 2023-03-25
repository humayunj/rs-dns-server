use std::net::UdpSocket;

use crate::{BytePacketBuffer, DnsPacket, DnsQuestion, QueryType, Result};

pub fn lookup(qname: &str, qtype: QueryType) -> Result<DnsPacket> {
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

    DnsPacket::from_buffer(&mut res_buffer)
}
