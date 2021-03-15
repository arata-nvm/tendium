use crate::protocol::link::address::MacAddress;
use std::io::{self, Read, Write};
use std::os::unix::io::AsRawFd;

use super::{sys, Device};

pub struct TunTap {
    dev: tun::platform::Device,
    name: String,
}

impl TunTap {
    pub fn new(name: String) -> tun::Result<TunTap> {
        let mut config = tun::Configuration::default();
        config.layer(tun::Layer::L2).name(&name).up();
        let dev = tun::create(&config)?;
        Ok(Self { dev, name })
    }
}

impl Device for TunTap {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn address(&self) -> io::Result<MacAddress> {
        sys::get_address(self.dev.as_raw_fd(), &self.name)
    }
}

impl Read for TunTap {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.dev.read(buf)
    }
}

impl Write for TunTap {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.dev.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.dev.flush()
    }
}
