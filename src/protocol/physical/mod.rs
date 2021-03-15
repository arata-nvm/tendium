use std::io::{self, Read, Write};

use super::link::ethernet::MacAddress;

pub mod raw_socket;
mod sys;
pub mod tuntap;

pub trait Device: Read + Write {
    fn name(&self) -> String;

    fn address(&self) -> io::Result<MacAddress>;
}
