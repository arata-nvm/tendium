use std::{
    fmt,
    io::{self, Read, Write},
    u16,
};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use super::{address::IPAddress, icmp::IcmpMessage};

// https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml
#[derive(Debug)]
pub struct IPDatagram {
    pub header: IPHeader,
    pub payload: IPPayload,
}

#[derive(Debug)]
pub struct IPHeader {
    pub version_ihl: u8,
    pub tos: u8,
    pub length: u16,
    pub identification: u16,
    pub flags_offset: u16,
    pub ttl: u8,
    pub protocol: Protocol,
    pub checksum: u16,
    pub src_addr: IPAddress,
    pub dst_addr: IPAddress,
}

#[derive(Debug)]
pub enum Protocol {
    Icmp,
    Tcp,
    Udp,
    Unknown(u8),
}

#[derive(Debug)]
pub enum IPPayload {
    Icmp(IcmpMessage),
    Raw(Vec<u8>),
}

impl IPDatagram {
    pub fn read_from<R: Read>(r: &mut R) -> io::Result<Self> {
        let header = IPHeader::read_from(r)?;
        let payload = match header.protocol {
            Protocol::Icmp => IPPayload::Icmp(IcmpMessage::read_from(r)?),
            _ => IPPayload::Raw({
                let mut v = Vec::new();
                r.read_to_end(&mut v)?;
                v
            }),
        };

        Ok(Self { header, payload })
    }

    pub fn write_to<W: Write>(self, w: &mut W) -> io::Result<()> {
        self.header.write_to(w)?;
        self.payload.write_to(w)?;
        Ok(())
    }
}

impl IPHeader {
    pub fn version(&self) -> u8 {
        (self.version_ihl & 0xf0) >> 4
    }

    pub fn ihl(&self) -> u8 {
        self.version_ihl & 0x0f
    }

    pub fn flags(&self) -> u16 {
        (self.flags_offset & 0xe000) >> 13
    }

    pub fn offset(&self) -> u16 {
        self.flags_offset & 0x1fff
    }

    pub fn read_from<R: Read>(r: &mut R) -> io::Result<Self> {
        Ok(Self {
            version_ihl: r.read_u8()?,
            tos: r.read_u8()?,
            length: r.read_u16::<BigEndian>()?,
            identification: r.read_u16::<BigEndian>()?,
            flags_offset: r.read_u16::<BigEndian>()?,
            ttl: r.read_u8()?,
            protocol: r.read_u8()?.into(),
            checksum: r.read_u16::<BigEndian>()?,
            src_addr: IPAddress::read_from(r)?,
            dst_addr: IPAddress::read_from(r)?,
        })
    }

    pub fn write_to<W: Write>(self, w: &mut W) -> io::Result<()> {
        w.write_u8(self.version_ihl)?;
        w.write_u8(self.tos)?;
        w.write_u16::<BigEndian>(self.length)?;
        w.write_u16::<BigEndian>(self.identification)?;
        w.write_u16::<BigEndian>(self.flags_offset)?;
        w.write_u8(self.ttl)?;
        w.write_u8(self.protocol.into())?;
        w.write_u16::<BigEndian>(self.checksum)?;
        w.write_all(&self.src_addr.0)?;
        w.write_all(&self.dst_addr.0)?;
        Ok(())
    }
}

impl IPPayload {
    pub fn write_to<W: Write>(self, w: &mut W) -> io::Result<()> {
        match self {
            Self::Icmp(icmp) => icmp.write_to(w),
            Self::Raw(v) => w.write_all(&v),
        }
    }

    pub fn protocol(&self) -> Protocol {
        match self {
            Self::Icmp(_) => Protocol::Icmp,
            _ => panic!(),
        }
    }
}

impl From<u8> for Protocol {
    fn from(v: u8) -> Self {
        match v {
            1 => Self::Icmp,
            6 => Self::Tcp,
            17 => Self::Udp,
            x => Self::Unknown(x),
        }
    }
}

impl From<Protocol> for u8 {
    fn from(v: Protocol) -> Self {
        match v {
            Protocol::Icmp => 1,
            Protocol::Tcp => 6,
            Protocol::Udp => 17,
            Protocol::Unknown(x) => x,
        }
    }
}

impl fmt::Display for IPHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "IPHeader:")?;
        writeln!(f, "  ver: {}", self.version())?;
        writeln!(f, "  ihl: {}", self.ihl())?;
        writeln!(f, "  tos: 0x{:x}", self.tos)?;
        writeln!(f, "  len: {}", self.length)?;
        writeln!(f, "  id:  {}", self.identification)?;
        writeln!(f, "  flg: 0b{:03b}", self.flags())?;
        writeln!(f, "  off: {}", self.offset())?;
        writeln!(f, "  ttl: {}", self.ttl)?;
        writeln!(f, "  pro: {}", self.protocol)?;
        writeln!(f, "  chs: 0x{:04x}", self.checksum)?;
        writeln!(f, "  src: {}", self.src_addr)?;
        write!(f, "  dst: {}", self.dst_addr)?;
        Ok(())
    }
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Protocol::*;
        match self {
            Icmp => write!(f, "ICMP(1)"),
            Tcp => write!(f, "TCP(6)"),
            Udp => write!(f, "UDP(17)"),
            Unknown(x) => write!(f, "Unknown({})", x),
        }
    }
}
