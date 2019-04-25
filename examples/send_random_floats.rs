use rand::Rng;
use std::thread;
use std::time::Duration;

///! An example to send random floats via FUDI over UDP
///! to a pure data patch every second.
extern crate fudi_rs; // add crate to talk to pure data

fn main() {
    println!("press CTRL + C to stop"); // print helpful hint

    // create new netsend with 127.0.0.1:39942 as destination for messages
    let netsend = fudi_rs::NetSendUdp::new("127.0.0.1:39942");

    // forever do ...
    loop {
        let f = rand::thread_rng().gen_range(-139.8, 694.5); // generate random number between -139.8 and 694.5
        let msg = fudi_rs::PdMessage::Float(f); // create a float message
        println!("sending {:?}", msg);
        netsend.send(&msg).expect("sending message failed"); // actually send the float message
        thread::sleep(Duration::from_secs(1)); // sleep for 1 second
    }
}
