use std::{
    fmt,
    io::{self, Read, Write},
};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

// https://www.iana.org/assignments/icmp-parameters/icmp-parameters.xhtml
#[derive(Debug)]
pub struct IcmpMessage {
    pub typ: IcmpType,
    pub code: u8,
    pub checksum: u16,
}

#[derive(Debug)]
pub enum IcmpType {
    EchoReply,
    DestinationUnreachable,
    Echo,
    TimeExceeded,
    Unknown(u8),
}

impl IcmpMessage {
    pub fn read_from<R: Read>(r: &mut R) -> io::Result<Self> {
        Ok(Self {
            typ: r.read_u8()?.into(),
            code: r.read_u8()?,
            checksum: r.read_u16::<BigEndian>()?,
        })
    }

    pub fn write_to<W: Write>(self, w: &mut W) -> io::Result<()> {
        w.write_u8(self.typ.into())?;
        w.write_u8(self.code)?;
        w.write_u16::<BigEndian>(self.checksum)?;
        Ok(())
    }
}

impl From<u8> for IcmpType {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::EchoReply,
            3 => Self::DestinationUnreachable,
            8 => Self::Echo,
            11 => Self::TimeExceeded,
            x => Self::Unknown(x),
        }
    }
}

impl From<IcmpType> for u8 {
    fn from(v: IcmpType) -> Self {
        match v {
            IcmpType::EchoReply => 0,
            IcmpType::DestinationUnreachable => 3,
            IcmpType::Echo => 8,
            IcmpType::TimeExceeded => 11,
            IcmpType::Unknown(x) => x,
        }
    }
}

impl fmt::Display for IcmpMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "IcmpMessage:")?;
        writeln!(f, "  typ: {}", self.typ)?;
        writeln!(f, "  cod: {}", self.code)?;
        write!(f, "  chs: {:04x}", self.checksum)?;
        Ok(())
    }
}

impl fmt::Display for IcmpType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use IcmpType::*;
        match self {
            EchoReply => write!(f, "Echo Reply(0)"),
            DestinationUnreachable => write!(f, "Destination Unreachable(3)"),
            Echo => write!(f, "Echo(8)"),
            TimeExceeded => write!(f, "Time Exceeded(11)"),
            Unknown(x) => write!(f, "Unknown({})", x),
        }
    }
}
