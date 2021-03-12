use std::{
    fmt,
    io::{self, Read},
};

use byteorder::ReadBytesExt;

#[derive(Debug, Clone)]
pub struct IPAddress(pub [u8; 4]);

impl IPAddress {
    pub fn read_from<R: Read>(r: &mut R) -> io::Result<Self> {
        let mut b = [0; 4];
        for i in 0..4 {
            b[i] = r.read_u8()?;
        }
        Ok(Self(b))
    }
}

impl fmt::Display for IPAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|i| format!("{}", i))
                .collect::<Vec<_>>()
                .join(".")
        )
    }
}
