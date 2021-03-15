use std::{
    fmt,
    io::{self, Read},
};

use byteorder::ReadBytesExt;

#[derive(Debug, Clone)]
pub struct MacAddress(pub [u8; 6]);

impl MacAddress {
    pub fn read_from<R: Read>(r: &mut R) -> io::Result<Self> {
        let mut b = [0; 6];
        for i in 0..6 {
            b[i] = r.read_u8()?;
        }
        Ok(Self(b))
    }
}

impl fmt::Display for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|i| format!("{:02x}", i))
                .collect::<Vec<_>>()
                .join(":")
        )
    }
}
