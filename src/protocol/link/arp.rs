use std::{
    fmt,
    io::{self, Read, Write},
};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::protocol::{internet::ip::IPAddress, link::ethernet::EtherType};

use super::ethernet::MacAddress;

// https://www.iana.org/assignments/arp-parameters/arp-parameters.xhtml
#[derive(Debug)]
pub struct Arp {
    pub hardware_type: HardwareType,
    pub protocol_type: EtherType,
    pub hardware_len: u8,
    pub protocol_len: u8,
    pub opcode: Opcode,
    pub sender_hardware_addr: MacAddress,
    pub sender_protocol_addr: IPAddress,
    pub target_hardware_addr: MacAddress,
    pub target_protocol_addr: IPAddress,
}

#[derive(Debug)]
pub enum HardwareType {
    Ethernet,
    Unknown(u16),
}

#[derive(Debug)]
pub enum Opcode {
    Request,
    Reply,
    Unknown(u16),
}

impl Arp {
    pub fn read_from<R: Read>(r: &mut R) -> io::Result<Self> {
        Ok(Self {
            hardware_type: r.read_u16::<BigEndian>()?.into(),
            protocol_type: r.read_u16::<BigEndian>()?.into(),
            hardware_len: r.read_u8()?,
            protocol_len: r.read_u8()?,
            opcode: r.read_u16::<BigEndian>()?.into(),
            sender_hardware_addr: MacAddress::read_from(r)?,
            sender_protocol_addr: IPAddress::read_from(r)?,
            target_hardware_addr: MacAddress::read_from(r)?,
            target_protocol_addr: IPAddress::read_from(r)?,
        })
    }

    pub fn write_to<W: Write>(self, w: &mut W) -> io::Result<()> {
        w.write_u16::<BigEndian>(self.hardware_type.into())?;
        w.write_u16::<BigEndian>(self.protocol_type.into())?;
        w.write_u8(self.hardware_len.into())?;
        w.write_u8(self.protocol_len.into())?;
        w.write_u16::<BigEndian>(self.opcode.into())?;
        w.write_all(&self.sender_hardware_addr.0)?;
        w.write_all(&self.sender_protocol_addr.0)?;
        w.write_all(&self.target_hardware_addr.0)?;
        w.write_all(&self.target_protocol_addr.0)?;
        Ok(())
    }
}

impl From<u16> for HardwareType {
    fn from(v: u16) -> Self {
        match v {
            1 => Self::Ethernet,
            x => Self::Unknown(x),
        }
    }
}

impl From<HardwareType> for u16 {
    fn from(v: HardwareType) -> Self {
        match v {
            HardwareType::Ethernet => 1,
            HardwareType::Unknown(x) => x,
        }
    }
}

impl From<u16> for Opcode {
    fn from(v: u16) -> Self {
        match v {
            1 => Self::Request,
            2 => Self::Reply,
            x => Self::Unknown(x),
        }
    }
}

impl From<Opcode> for u16 {
    fn from(v: Opcode) -> Self {
        match v {
            Opcode::Request => 1,
            Opcode::Reply => 2,
            Opcode::Unknown(x) => x,
        }
    }
}

impl fmt::Display for Arp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Arp:")?;
        writeln!(f, "  hrd: {}", self.hardware_type)?;
        writeln!(f, "  pro: {}", self.protocol_type)?;
        writeln!(f, "  hln: {}", self.hardware_len)?;
        writeln!(f, "  pln: {}", self.protocol_len)?;
        writeln!(f, "  op: {}", self.opcode)?;
        writeln!(f, "  sha: {}", self.sender_hardware_addr)?;
        writeln!(f, "  spa: {}", self.sender_protocol_addr)?;
        writeln!(f, "  tha: {}", self.target_hardware_addr)?;
        write!(f, "  tpa: {}", self.target_protocol_addr)?;
        Ok(())
    }
}

impl fmt::Display for HardwareType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use HardwareType::*;
        match self {
            Ethernet => write!(f, "Ethernet(0x1)"),
            Unknown(x) => write!(f, "Unknown(0x{:x})", x),
        }
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Opcode::*;
        match self {
            Request => write!(f, "Request(0x1)"),
            Reply => write!(f, "Reply(0x2)"),
            Unknown(x) => write!(f, "Unknown(0x{:x})", x),
        }
    }
}
