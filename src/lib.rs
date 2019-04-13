use std::str::FromStr;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::io::{Result};

/// An incomplete implementation of Pure Data message types.
/// TODO: implement list, pointer and custom message types
enum PdMessage {
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
	payload = format!("{};\n", payload);
	return payload;
    }
}


// easier than an enum (for later matching)
struct NetSendUdp {
    target: SocketAddr,
    socket: UdpSocket,
}

impl NetSendUdp {
    /// Create a new instance and set target address.
    fn new(target: &str) -> crate::NetSendUdp {
       NetSendUdp {
           target: SocketAddr::from_str(target).expect("failed to parse target address"),
	   socket: UdpSocket::bind("0.0.0.0:0").expect("failed to bind host socket"),
       }
    }

    /// Send a message to the target.
    fn send(&self, msg: PdMessage) -> Result<usize> {
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
	assert_eq!(ns.target.ip(), IpAddr::V4(Ipv4Addr::new(127,0,0,1)));
	assert_eq!(ns.target.port(), 8989);
    }

    #[test]
    fn send_into_ether() {
        let msg = PdMessage::Bang;
	let target = "127.0.0.1:8989";
        let ns = NetSendUdp::new(&String::from(target));
	let res = ns.send(msg);
	match res {
	    Ok(bsend) => assert_eq!(bsend, 6),
	    Err(fail) => panic!(fail),
	}
    }
}
