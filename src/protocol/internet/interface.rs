use std::io::{self, Read, Write};

use link::ethernet::EthernetPayload;

use crate::protocol::{
    link::{self, address::MacAddress, arp},
    physical::Device,
};

use super::{
    address::IPAddress,
    ip::{IPDatagram, IPHeader, IPPayload, Protocol},
};

pub struct Interface {
    dev: link::Interface,
    ip_addr: IPAddress,
    arp_table: arp::ArpTable,
}

impl Interface {
    pub fn new(dev: link::Interface, ip_addr: IPAddress) -> io::Result<Self> {
        Ok(Self {
            dev,
            ip_addr,
            arp_table: arp::ArpTable::new(),
        })
    }

    pub fn mac_addr(&self) -> &MacAddress {
        self.dev.mac_addr()
    }

    pub fn ip_addr(&self) -> &IPAddress {
        &self.ip_addr
    }

    pub fn link(&mut self) -> &mut link::Interface {
        &mut self.dev
    }

    pub fn recv(&mut self) -> io::Result<IPDatagram> {
        loop {
            let frame = self.dev.recv()?;
            if let EthernetPayload::IP(ip) = frame.payload {
                return Ok(ip);
            }
        }
    }

    pub fn send(&mut self, dst_addr: IPAddress, payload: IPPayload) -> io::Result<()> {
        let header = IPHeader {
            version_ihl: (4 << 4) | 5,
            tos: 5,
            length: 112,
            identification: 0,
            flags_offset: 0,
            ttl: 64,
            protocol: payload.protocol(),
            checksum: 0,
            src_addr: self.ip_addr().clone(),
            dst_addr,
        };
        let datagram = IPDatagram { header, payload };
        let dst_mac_addr = self.arp_table.find(dst_addr, &mut self).unwrap();
        self.dev
            .send(self.mac_addr().clone(), EthernetPayload::IP(datagram))
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
