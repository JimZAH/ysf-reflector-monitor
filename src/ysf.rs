use crate::Config;
use std::{net, sync::Arc, thread, time};
const YSF_BUFF: usize = 155;

pub struct Ysf {
    pub route: String,
    pub callsign: String,
    pub destination: String,
    pub packet_count: u8,
}

impl Ysf {
    fn parse(buf: &[u8; YSF_BUFF]) -> Option<Self> {
        if !buf[..4].iter().eq([b'Y', b'S', b'F', b'D'].iter()) {
            return None;
        }
        let y = Ysf {
            route: std::str::from_utf8(&buf[4..14]).unwrap().trim().to_string(),
            callsign: std::str::from_utf8(&buf[14..20])
                .unwrap()
                .trim()
                .to_string(),
            destination: std::str::from_utf8(&buf[20..30])
                .unwrap()
                .trim()
                .to_string(),
            packet_count: buf[34],
        };
        Some(y)
    }
}

pub fn run(config: Config) {
    let mut call = "YSFP ".to_string();
    call.push_str(&config.callsign);
    let sock = Arc::new(net::UdpSocket::bind(&config.bind).unwrap());
    let sox = sock.clone();
    thread::spawn(move || loop {
        sox.send_to(call.as_bytes(), &config.server).unwrap();
        thread::sleep(time::Duration::from_millis(1000 * 5));
    });

    let mut buf: [u8; 155] = [0; 155];
    loop {
        let byte_count = sock.recv(&mut buf).unwrap();
        if byte_count < 155 {
            continue;
        }
        if let Some(y_frame) = Ysf::parse(&buf) {
            if y_frame.route == "REFLECTOR" {
                continue;
            }
            println!(
                "Route: {}\nCall: {}\nDest: {}\nPacket_count: {}\n",
                y_frame.route, y_frame.callsign, y_frame.destination, y_frame.packet_count
            );
        }
    }
}
