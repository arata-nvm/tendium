use std::{
    fmt,
    io::{self, Read, Write},
};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::protocol::{internet::ip::IPDatagram, link::arp::Arp};

use super::address::MacAddress;

#[derive(Debug)]
pub struct EthernetFrame {
    pub header: EthernetHeader,
    pub payload: EthernetPayload,
}

// https://www.iana.org/assignments/ieee-802-numbers/ieee-802-numbers.xhtml
#[derive(Debug)]
pub struct EthernetHeader {
    pub dst_addr: MacAddress,
    pub src_addr: MacAddress,
    pub typ: EtherType,
}

#[derive(Debug)]
pub enum EtherType {
    IPv4,
    Arp,
    Unknown(u16),
}

#[derive(Debug)]
pub enum EthernetPayload {
    Arp(Arp),
    IP(IPDatagram),
    Raw(Vec<u8>),
}

impl EthernetFrame {
    pub fn read_from<R: Read>(r: &mut R) -> io::Result<Self> {
        let header = EthernetHeader {
            dst_addr: MacAddress::read_from(r)?,
            src_addr: MacAddress::read_from(r)?,
            typ: r.read_u16::<BigEndian>()?.into(),
        };

        let payload = match header.typ {
            EtherType::Arp => EthernetPayload::Arp(Arp::read_from(r)?),
            EtherType::IPv4 => EthernetPayload::IP(IPDatagram::read_from(r)?),
            _ => EthernetPayload::Raw({
                let mut v = Vec::new();
                r.read_to_end(&mut v)?;
                v
            }),
        };

        Ok(Self { header, payload })
    }

    pub fn write_to<W: Write>(self, w: &mut W) -> io::Result<()> {
        w.write_all(&self.header.dst_addr.0)?;
        w.write_all(&self.header.src_addr.0)?;
        w.write_u16::<BigEndian>(self.header.typ.into())?;
        self.payload.write_to(w)?;
        Ok(())
    }
}

impl EthernetPayload {
    pub fn write_to<W: Write>(self, w: &mut W) -> io::Result<()> {
        match self {
            Self::Arp(arp) => arp.write_to(w),
            Self::IP(ip) => ip.write_to(w),
            Self::Raw(v) => w.write_all(&v),
        }
    }

    pub fn typ(&self) -> EtherType {
        match self {
            Self::Arp(_) => EtherType::Arp,
            Self::IP(_) => EtherType::IPv4,
            // TODO
            Self::Raw(_) => panic!(),
        }
    }
}

impl From<u16> for EtherType {
    fn from(v: u16) -> Self {
        match v {
            0x0800 => Self::IPv4,
            0x0806 => Self::Arp,
            x => Self::Unknown(x),
        }
    }
}

impl From<EtherType> for u16 {
    fn from(v: EtherType) -> Self {
        match v {
            EtherType::IPv4 => 0x0800,
            EtherType::Arp => 0x0806,
            EtherType::Unknown(x) => x,
        }
    }
}

impl fmt::Display for EthernetHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "EthernetHeader:")?;
        writeln!(f, "  dst: {}", self.dst_addr)?;
        writeln!(f, "  src: {}", self.src_addr)?;
        write!(f, "  typ: {}", self.typ)?;
        Ok(())
    }
}

impl fmt::Display for EtherType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use EtherType::*;
        match self {
            IPv4 => write!(f, "IPv4(0x0800)"),
            Arp => write!(f, "ARP(0x0806)"),
            Unknown(x) => write!(f, "UNKNOWN(0x{:04x})", x),
        }
    }
}
