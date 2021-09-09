use std::{env, io};

use tendium::protocol::{
    dump, link,
    physical::{raw_socket::RawSocket, Device},
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <ifname>", args[0]);
        return Ok(());
    }

    let dev = RawSocket::new(args[1].clone())?;
    println!("[{}] {}", dev.name(), dev.address()?);

    let mut link_iface = link::Interface::new(Box::new(dev))?;
    loop {
        let frame = link_iface.recv()?;
        println!("--- [{}] ---", link_iface.name());
        dump(&frame);
        println!();
    }
}
