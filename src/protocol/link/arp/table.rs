use std::collections::HashMap;

use crate::protocol::{
    internet::{self, address::IPAddress},
    link::{
        self,
        address::MacAddress,
        arp,
        ethernet::{EtherType, EthernetPayload},
    },
};

use super::{Arp, HardwareType};

#[derive(Debug)]
pub struct ArpTable(HashMap<IPAddress, MacAddress>);

impl ArpTable {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get(&self, addr: &IPAddress) -> Option<MacAddress> {
        self.0.get(addr).cloned()
    }

    pub fn insert(&mut self, ip_addr: IPAddress, mac_addr: MacAddress) {
        self.0.insert(ip_addr, mac_addr);
    }

    pub fn find(&mut self, addr: IPAddress, iface: &mut internet::Interface) -> Option<MacAddress> {
        if self.0.contains_key(&addr) {
            return self.get(&addr);
        }

        let payload = EthernetPayload::Arp(Arp::new(
            arp::Opcode::Request,
            iface.mac_addr().clone(),
            iface.ip_addr().clone(),
            MacAddress::broadcast(),
            addr.clone(),
        ));

        iface.link().send(MacAddress::broadcast(), payload).unwrap();

        loop {
            let frame = iface.link().recv().unwrap();
            if let EthernetPayload::Arp(arp) = frame.payload {
                if arp.opcode == arp::Opcode::Reply {
                    self.insert(arp.sender_protocol_addr, arp.sender_hardware_addr);
                    return self.get(&addr);
                }
            }
        }
    }
}
