pub mod internet;
pub mod link;
pub mod physical;

use internet::ip::IPPayload;
use link::ethernet::EthernetFrame;
use link::ethernet::EthernetPayload;

pub fn dump(frame: &EthernetFrame) {
    println!("{}", frame.header);
    match &frame.payload {
        EthernetPayload::Arp(arp) => println!("{}", arp),
        EthernetPayload::IP(ip) => {
            println!("{}", ip.header);
            match &ip.payload {
                IPPayload::Icmp(icmp) => println!("{}", icmp),
                _ => {}
            }
        }
        _ => {}
    }
}
