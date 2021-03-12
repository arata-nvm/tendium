use std::{
    env,
    io::{self, Cursor},
};

use tendium::protocol::{
    link::ethernet::{EthernetFrame, EthernetPayload},
    physical::raw_socket::RawSocket,
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <ifname>", args[0]);
        return Ok(());
    }

    let raw_socket = setup(args[1].clone())?;
    println!(
        "[{}] {:x?}",
        raw_socket.name,
        raw_socket.get_hardware_addr()?
    );

    let mut buf = [0; 1024];
    loop {
        let len = raw_socket.recv(&mut buf)?;
        let mut cursor = Cursor::new(&buf[..len]);
        let frame = EthernetFrame::read_from(&mut cursor)?;

        println!("--- [{}] {} bytes ---", raw_socket.name, len);
        println!("{}", frame.header);

        match frame.payload {
            EthernetPayload::Arp(arp) => println!("{}", arp),
            _ => {}
        }
    }
}

fn setup(name: String) -> io::Result<RawSocket> {
    let raw_socket = RawSocket::new(name)?;
    raw_socket.bind()?;
    raw_socket.set_promisc()?;
    Ok(raw_socket)
}
