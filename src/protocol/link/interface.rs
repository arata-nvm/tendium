use std::io::{self, Cursor, Read, Write};

use crate::protocol::physical::Device;

use super::{
    address::MacAddress,
    ethernet::{EthernetFrame, EthernetHeader, EthernetPayload},
};

pub struct Interface {
    dev: Box<dyn Device>,
    mac_addr: MacAddress,
}

impl Interface {
    pub fn new(dev: Box<dyn Device>) -> io::Result<Self> {
        Ok(Self {
            mac_addr: dev.address()?,
            dev,
        })
    }

    pub fn mac_addr(&self) -> &MacAddress {
        &self.mac_addr
    }

    pub fn recv(&mut self) -> io::Result<EthernetFrame> {
        let mut buf = [0; 4096];
        let len = self.dev.read(&mut buf).unwrap();

        let mut cursor = Cursor::new(&buf[0..len]);
        let frame = EthernetFrame::read_from(&mut cursor)?;
        Ok(frame)
    }

    pub fn send(&mut self, dst_addr: MacAddress, payload: EthernetPayload) -> io::Result<()> {
        let header = EthernetHeader {
            dst_addr,
            src_addr: self.mac_addr().clone(),
            typ: payload.typ(),
        };

        let frame = EthernetFrame { header, payload };
        frame.write_to(&mut self.dev)
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
