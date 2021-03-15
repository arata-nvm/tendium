use std::{io, mem};

use crate::protocol::link::address::MacAddress;

pub fn get_address(fd: i32, name: &str) -> io::Result<MacAddress> {
    unsafe {
        let mut ifreq: ifstructs::ifreq = mem::zeroed();
        ifreq.set_name(name)?;

        if libc::ioctl(fd, libc::SIOCGIFHWADDR, &ifreq) == -1 {
            return Err(io::Error::last_os_error());
        }

        let mut addr = [0u8; 6];
        addr.clone_from_slice(
            &ifreq.ifr_ifru.ifr_hwaddr.sa_data[..6]
                .into_iter()
                .map(|&i| i as u8)
                .collect::<Vec<_>>(),
        );
        Ok(MacAddress(addr))
    }
}
