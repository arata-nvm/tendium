use std::{
    env,
    io::{self, Read},
};

use tendium::protocol::physical::{raw_socket::RawSocket, Device};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <ifname>", args[0]);
        return Ok(());
    }

    let mut dev = RawSocket::new(args[1].clone())?;
    println!("[{}] {}", dev.name(), dev.address()?);

    let mut buf = [0; 1024];
    loop {
        let len = dev.read(&mut buf)?;
        println!("--- [{}] {} bytes ---", dev.name(), len);
        println!("{:?}", &buf[..len]);
    }
}
