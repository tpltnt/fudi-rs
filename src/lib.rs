//! This crate enables the communication between Rust programs
//! and Pure Data over a network using the FUDI protocol.
// !
//! # references
//! * [Pure Data](http://puredata.info/)
//! * [FUDI specification](https://web.archive.org/web/20120304071510/http://wiki.puredata.info/en/FUDI) (via archive.org)
//! * [wikipedia: FUDI](https://en.wikipedia.org/wiki/FUDI)
//! * [FLOSS Manuals: Pure Data - messages](http://write.flossmanuals.net/pure-data/messages/)
//! * [FLOSS manuals: Pure Data - send and receive](http://write.flossmanuals.net/pure-data/send-and-receive/)

use std::io::Result;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::str::FromStr;

/// An incomplete implementation of Pure Data message types.
///
/// # implemented
/// * Float messages
/// * Symbol messages (based on strings)
/// * Bang messages
///
/// # not implemented
/// * list
/// * pointer
/// * custom message
///
/// # references
/// * [FLOSS Manuals: Pure Data - messages](http://write.flossmanuals.net/pure-data/messages/)
pub enum PdMessage {
    Float(f32),
    Symbol(String),
    Bang,
}

impl PdMessage {
    /// Generate a message string for the (given) message type
    fn to_text(&self) -> String {
        let mut payload: String;
        match &self {
            PdMessage::Float(f) => payload = format!("float {}", f),
            PdMessage::Symbol(word) => payload = format!("symbol {}", word),
            PdMessage::Bang => payload = String::from("bang"),
        }
        payload = format!("{};\n", payload); // newline not in spec, but in vanilla pd
        return payload;
    }
}

/// Encapsulate sending Pure Date messages via FUDI over UDP.
/// This is the library equivalent of the netsend-object for UDP.
///
/// # references
/// * [FLOSS manuals: Pure Data - send and receive](http://write.flossmanuals.net/pure-data/send-and-receive/)
pub struct NetSendUdp {
    target: SocketAddr,
    socket: UdpSocket,
}

impl NetSendUdp {
    /// Create a new instance and set target address.
    ///
    /// # Arguments
    /// * `target` - target host (& port) to send messages to
    pub fn new(target: &str) -> crate::NetSendUdp {
        NetSendUdp {
            target: SocketAddr::from_str(target).expect("failed to parse target address"),
            socket: UdpSocket::bind("0.0.0.0:0").expect("failed to bind host socket"),
        }
    }

    /// Send a message to the target and return the number of bytes sent.
    ///
    /// # Arguments
    /// * `msg` - message to send to the target
    pub fn send(&self, msg: &PdMessage) -> Result<usize> {
        self.socket.send_to(msg.to_text().as_bytes(), self.target)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generate_float_message() {
        let msg = PdMessage::Float(2.974);
        assert_eq!(String::from("float 2.974;\n"), msg.to_text());
    }

    #[test]
    fn generate_symbol_message() {
        let msg = PdMessage::Symbol(String::from("foobar"));
        assert_eq!(String::from("symbol foobar;\n"), msg.to_text());
    }

    #[test]
    fn generate_bang_message() {
        let msg = PdMessage::Bang;
        assert_eq!(String::from("bang;\n"), msg.to_text());
    }

    #[test]
    fn create_udp_netsend_test_target() {
        let target = "127.0.0.1:8989";
        let ns = NetSendUdp::new(&String::from(target));

        assert_eq!(ns.target.is_ipv4(), true);
        assert_eq!(ns.target.ip(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
        assert_eq!(ns.target.port(), 8989);
    }

    #[test]
    fn send_bang_into_ether() {
        let msg = PdMessage::Bang;
        let target = "127.0.0.1:8989";
        let ns = NetSendUdp::new(&String::from(target));
        let res = ns.send(&msg);
        match res {
            Ok(bsend) => assert_eq!(bsend, 6),
            Err(fail) => panic!(fail),
        }
    }

    #[test]
    fn send_float_into_ether() {
        let msg = PdMessage::Float(432.0);
        let target = "127.0.0.1:8989";
        let ns = NetSendUdp::new(&String::from(target));
        let res = ns.send(&msg);
        match res {
            Ok(bsend) => assert_eq!(bsend, 11),
            Err(fail) => panic!(fail),
        }
    }
}
