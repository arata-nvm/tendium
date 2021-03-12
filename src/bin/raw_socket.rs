use std::{env, io};

use tendium::protocol::physical::raw_socket::RawSocket;

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
        println!("--- [{}] {} bytes ---", raw_socket.name, len);
        println!("{:?}", &buf[..len]);
    }
}

fn setup(name: String) -> io::Result<RawSocket> {
    let raw_socket = RawSocket::new(name)?;
    raw_socket.bind()?;
    raw_socket.set_promisc()?;
    Ok(raw_socket)
}
