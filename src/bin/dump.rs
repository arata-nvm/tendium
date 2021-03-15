use std::{
    env,
    io::{self, Cursor, Read},
};

use tendium::protocol::{
    internet::ip::IPPayload,
    link::ethernet::{EthernetFrame, EthernetPayload},
    physical::{raw_socket::RawSocket, Device},
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <ifname>", args[0]);
        return Ok(());
    }

    let mut dev = RawSocket::new(args[1].clone())?;
    println!("[{}] {}", dev.name(), dev.address()?);

    let mut buf = [0; 4096];
    loop {
        let len = dev.read(&mut buf)?;
        let mut cursor = Cursor::new(&buf[..len]);
        let frame = EthernetFrame::read_from(&mut cursor)?;

        println!("--- [{}] {} bytes ---", dev.name(), len);
        println!("{}", frame.header);
        match frame.payload {
            EthernetPayload::Arp(arp) => println!("{}", arp),
            EthernetPayload::IP(ip) => {
                println!("{}", ip.header);
                match ip.payload {
                    IPPayload::Icmp(icmp) => println!("{}", icmp),
                    _ => {}
                }
            }
            _ => {}
        }
        println!();
    }
}
