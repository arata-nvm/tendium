use std::{ffi::CString, io, mem};

use libc;

pub struct RawSocket {
    fd: i32,
    pub name: String,
}

extern "C" {
    fn htons(hostshort: i32) -> i32;
}

impl RawSocket {
    pub fn new(name: String) -> io::Result<RawSocket> {
        let fd = unsafe {
            match libc::socket(
                libc::AF_PACKET,
                libc::SOCK_RAW,
                libc::ETH_P_ALL.to_be() as i32,
            ) {
                -1 => return Err(io::Error::last_os_error()),
                fd => fd,
            }
        };

        Ok(RawSocket { fd, name })
    }

    pub fn bind(&self) -> io::Result<()> {
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
        let name_cstr = CString::new(self.name.clone()).unwrap();
        unsafe {
            match libc::if_nametoindex(name_cstr.as_ptr() as *const i8) {
                0 => Err(io::Error::last_os_error()),
                x => Ok(x),
            }
        }
    }

    pub fn set_promisc(&self) -> io::Result<()> {
        unsafe {
            let mut ifreq: ifstructs::ifreq = mem::zeroed();
            ifreq.set_name(&self.name)?;

            if libc::ioctl(self.fd, libc::SIOCGIFFLAGS, &ifreq) == -1 {
                return Err(io::Error::last_os_error());
            }

            ifreq.ifr_ifru.ifr_flags |= libc::IFF_PROMISC as i16;

            if libc::ioctl(self.fd, libc::SIOCSIFFLAGS, &ifreq) == -1 {
                return Err(io::Error::last_os_error());
            }

            Ok(())
        }
    }

    pub fn get_hardware_addr(&self) -> io::Result<Vec<i8>> {
        unsafe {
            let mut ifreq: ifstructs::ifreq = mem::zeroed();
            ifreq.set_name(&self.name)?;

            if libc::ioctl(self.fd, libc::SIOCGIFHWADDR, &ifreq) == -1 {
                return Err(io::Error::last_os_error());
            }

            Ok(ifreq.ifr_ifru.ifr_hwaddr.sa_data[..6].to_vec())
        }
    }

    pub fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
        unsafe {
            match libc::recv(self.fd, buf.as_mut_ptr() as *mut libc::c_void, buf.len(), 0) {
                -1 => Err(io::Error::last_os_error()),
                len => Ok(len as usize),
            }
        }
    }

    pub fn send(&self, buf: &[u8]) -> io::Result<usize> {
        unsafe {
            match libc::send(self.fd, buf.as_ptr() as *const libc::c_void, buf.len(), 0) {
                -1 => Err(io::Error::last_os_error()),
                len => Ok(len as usize),
            }
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
