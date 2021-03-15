use crate::protocol::link::address::MacAddress;
use libc;
use std::{
    ffi::CString,
    io::{self, Read, Write},
    mem,
};

use super::{sys, Device};

extern "C" {
    fn htons(hostshort: i32) -> i32;
}

pub struct RawSocket {
    fd: i32,
    name: String,
}

impl RawSocket {
    pub fn new(name: String) -> io::Result<RawSocket> {
        unsafe {
            let fd = match libc::socket(
                libc::AF_PACKET,
                libc::SOCK_RAW,
                libc::ETH_P_ALL.to_be() as i32,
            ) {
                -1 => return Err(io::Error::last_os_error()),
                fd => fd,
            };
            let raw_socket = RawSocket { fd, name };
            raw_socket.bind()?;

            Ok(raw_socket)
        }
    }

    fn bind(&self) -> io::Result<()> {
        unsafe {
            let mut addr: libc::sockaddr_ll = mem::zeroed();
            addr.sll_family = libc::AF_PACKET as u16;
            addr.sll_protocol = htons(libc::ETH_P_ALL) as u16;
            addr.sll_ifindex = self.get_if_index()? as i32;

            match libc::bind(
                self.fd,
                &addr as *const _ as *const libc::sockaddr,
                mem::size_of::<libc::sockaddr_ll>() as u32,
            ) {
                -1 => Err(io::Error::last_os_error()),
                _ => Ok(()),
            }
        }
    }

    fn get_if_index(&self) -> io::Result<u32> {
        unsafe {
            let name_cstr = CString::new(self.name.clone()).unwrap();
            match libc::if_nametoindex(name_cstr.as_ptr() as *const i8) {
                0 => Err(io::Error::last_os_error()),
                x => Ok(x),
            }
        }
    }
}

impl Device for RawSocket {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn address(&self) -> io::Result<MacAddress> {
        sys::get_address(self.fd, &self.name)
    }
}

impl Read for RawSocket {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        unsafe {
            match libc::recv(self.fd, buf.as_mut_ptr() as *mut libc::c_void, buf.len(), 0) {
                -1 => Err(io::Error::last_os_error()),
                len => Ok(len as usize),
            }
        }
    }
}

impl Write for RawSocket {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        unsafe {
            match libc::send(self.fd, buf.as_ptr() as *const libc::c_void, buf.len(), 0) {
                -1 => Err(io::Error::last_os_error()),
                len => Ok(len as usize),
            }
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        unsafe {
            let mode = CString::new("w").unwrap();
            let file = libc::fdopen(self.fd, mode.as_ptr());
            libc::fflush(file);
            Ok(())
        }
    }
}

impl Drop for RawSocket {
    fn drop(&mut self) {
        unsafe {
            libc::close(self.fd);
        }
    }
}
