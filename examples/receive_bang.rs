///! An example to receive bangs via FUDI over UDP
///! from a pure data patch.
extern crate fudi_rs; // add crate to talk to pure data

fn main() {
    println!("press CTRL + C to stop"); // print helpful hint

    // create new netreceive and listen on 127.0.0.1:18538 for messages
    let netreceive = fudi_rs::NetReceiveUdp::new("127.0.0.1:18538");

    // forever do ...
    loop {
        let msg = netreceive.receive();
        match msg {
            Ok(b) => println!("received {:?}", b),
            Err(e) => panic!("{}", e),
        }
    }
}
