use std::io::{Cursor, Read, Write};

use tendium::protocol::{
    internet::ip::IPAddress,
    link::ethernet::{EtherType, EthernetFrame, EthernetHeader, EthernetPayload, MacAddress},
    physical::tuntap::TunTap,
};

fn main() -> tun::Result<()> {
    let mac_addr = MacAddress([0x44, 0xc4, 0xc3, 0xf1, 0x15, 0x5b]);
    let ip_addr = IPAddress([10, 0, 0, 4]);

    let mut dev = TunTap::new("tap0".into())?;
    let mut buf = [0; 4096];

    loop {
        let len = dev.read(&mut buf).unwrap();

        let mut cursor = Cursor::new(&buf[0..len]);
        let frame = EthernetFrame::read_from(&mut cursor)?;

        println!("--- [tap0] {} bytes ---", len);

        match frame.payload {
            EthernetPayload::Arp(mut arp) => {
                println!("Received:");
                println!("{}", frame.header);
                println!("{}", arp);
                arp.target_hardware_addr = arp.sender_hardware_addr;
                arp.target_protocol_addr = arp.sender_protocol_addr;
                arp.sender_hardware_addr = mac_addr.clone();
                arp.sender_protocol_addr = ip_addr.clone();
                arp.opcode = tendium::protocol::link::arp::Opcode::Reply;

                let new_frame_header = EthernetHeader {
                    dst_addr: frame.header.src_addr,
                    src_addr: mac_addr.clone(),
                    typ: EtherType::Arp,
                };
                println!("Sent:");
                println!("{}", new_frame_header);
                println!("{}", arp);

                let new_frame = EthernetFrame {
                    header: new_frame_header,
                    payload: EthernetPayload::Arp(arp),
                };
                let mut raw_frame = Vec::new();
                new_frame.write_to(&mut raw_frame)?;
                dev.write_all(&mut raw_frame)?;
            }
            _ => println!("==> unknown type. dropping..."),
        }
    }
}
