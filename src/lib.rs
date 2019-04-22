//! This crate enables the communication between Rust programs
//! and Pure Data over a network using the FUDI protocol.
//!
//! # Examples
//! Create and send a bang to a Pure Data instance with a netreceive object listening
//! on 127.0.0.1:5678 for UDP traffic.
//! ```rust
//! let netsend = fudi_rs::NetSendUdp::new("127.0.0.1:5678");
//! let msg = fudi_rs::PdMessage::Bang;
//! netsend.send(&msg).expect("sending message failed");
//! ```
//!
//! # References
//! * [Pure Data](http://puredata.info/)
//! * [FUDI specification](https://web.archive.org/web/20120304071510/http://wiki.puredata.info/en/FUDI) (via archive.org)
//! * [wikipedia: FUDI](https://en.wikipedia.org/wiki/FUDI)
//! * [FLOSS Manuals: Pure Data - messages](http://write.flossmanuals.net/pure-data/messages/)
//! * [FLOSS manuals: Pure Data - send and receive](http://write.flossmanuals.net/pure-data/send-and-receive/)

use std::io::Result;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::str::FromStr;

#[macro_use]
extern crate nom;

mod parser;

/// An implementation of the most generic Pure Data message type.
#[derive(Debug)]
pub struct GenericMessage {
    selector: String,
    atoms: Vec<String>,
}

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
/// # Examples
/// Create a message to send a (floating point) number.
/// ```rust
/// let msg = fudi_rs::PdMessage::Float(23.42);
/// ```
///
/// # references
/// * [FLOSS Manuals: Pure Data - messages](http://write.flossmanuals.net/pure-data/messages/)
/// * [puredata.info: PdMessages](https://puredata.info/dev/PdMessages)
#[derive(Debug)]
pub enum PdMessage {
    Float(f32),
    Symbol(String),
    Bang,
    Generic(GenericMessage),
}

impl PdMessage {
    /// Generate a message string for the (given) message type.
    /// # note
    /// A message needs a trailing newline (i.e. '\n') according to the Java example in the [old wiki page](https://web.archive.org/web/20120304071510/http://wiki.puredata.info/en/FUDI). This is not explicitly mentioned in the FUDI specification.
    fn to_text(&self) -> String {
        let mut payload: String;
        match &self {
            PdMessage::Float(f) => payload = format!("float {}", f),
            PdMessage::Symbol(word) => payload = format!("symbol {}", word),
            PdMessage::Bang => payload = String::from("bang"),
            PdMessage::Generic(msg) => {
                payload = msg.selector.clone();
                for atom in msg.atoms.iter() {
                    payload = payload + " " + atom;
                }
            }
        }
        payload = format!("{};\n", payload); // newline not in spec, but in vanilla pd
        return payload;
    }
}

#[cfg(test)]
mod test_pdmessage {
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
    fn generate_generic_message() {
        let msg = PdMessage::Generic(GenericMessage {
            selector: String::from("selector"),
            atoms: vec!["one".to_string(), "two".to_string(), "17.9".to_string()],
        });
        assert_eq!(String::from("selector one two 17.9;\n"), msg.to_text());
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
mod test_netsendudp {
    use super::*;

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

/// Encapsulate receiving Pure Date messages via FUDI over UDP.
/// This is the library equivalent of the netreceive-object for UDP.
///
/// # references
/// * [FLOSS manuals: Pure Data - send and receive](http://write.flossmanuals.net/pure-data/send-and-receive/)
pub struct NetReceiveUdp {
    socket: UdpSocket,
}

impl NetReceiveUdp {
    /// Create a new instance and set address to listen on.
    ///
    /// # Arguments
    /// * `addr` - host (& port) to listen for messages
    pub fn new(addr: &str) -> crate::NetReceiveUdp {
        let laddr = SocketAddr::from_str(addr).expect("failed to parse target address");
        NetReceiveUdp {
            socket: UdpSocket::bind(laddr).expect("failed to bind socket to host"),
        }
    }

    /// Receive binary data via UDP.
    ///
    /// *note*: This function panics upon errors.
    pub fn receive_binary(&self) -> Vec<u8> {
        // max 65,507 bytes (65,535 − 8 byte UDP header − 20 byte IP header)
        let mut buffer: [u8; 1] = [0; 1];
        let recv_result = self.socket.recv_from(&mut buffer);
        let mut data;
        match recv_result {
            Ok((amount, _)) => data = Vec::from(&buffer[..amount]),
            Err(e) => panic!("receiving data failed: {:?}", e),
        }
        data
    }
}

#[cfg(test)]
mod test_netreceiveudp {
    use super::*;

    #[test]
    fn create_udp_netreceiveudp_test_target() {
        // create netreceive
        let target = "127.0.0.1:8989";
        let nr = NetReceiveUdp::new(&String::from(target));

        // extract socket from netreceive
        let nr_socket = nr
            .socket
            .local_addr()
            .expect("could not retrieve socket address");

        // test properties
        assert_eq!(nr_socket.is_ipv4(), true);
        assert_eq!(nr_socket.ip(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
        assert_eq!(nr_socket.port(), 8989);
    }
}
