use std::io::{self, Read, Write};

use crate::protocol::{
    link::{self, address::MacAddress},
    physical::Device,
};

use super::address::IPAddress;

pub struct Interface {
    dev: link::Interface,
    ip_addr: IPAddress,
}

impl Interface {
    pub fn new(dev: link::Interface, ip_addr: IPAddress) -> io::Result<Self> {
        Ok(Self { dev, ip_addr })
    }

    pub fn mac_addr(&self) -> &MacAddress {
        self.dev.mac_addr()
    }

    pub fn ip_addr(&self) -> &IPAddress {
        &self.ip_addr
    }
}

impl Device for Interface {
    fn name(&self) -> String {
        self.dev.name()
    }

    fn address(&self) -> io::Result<MacAddress> {
        self.dev.address()
    }
}

impl Read for Interface {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.dev.read(buf)
    }
}

impl Write for Interface {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.dev.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.dev.flush()
    }
}
