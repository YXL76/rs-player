use std::net::TcpListener;
use std::thread::spawn;

use json;
use tungstenite::server::accept;

pub fn init_server(port: &str) {
    let address = "127.0.0.1";
    let server = TcpListener::bind(format!("{}:{}", address, port)).unwrap();
    for stream in server.incoming() {
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();

            loop {
                let msg = websocket.read_message().unwrap();
                if msg.is_text() {
                    if let Ok(msg_) = msg.to_text() {
                        if let Ok(parsed_) = json::parse(msg_) {
                            println!("{}", parsed_);
                            websocket.write_message(msg).unwrap();
                        }
                    }
                }
            }
        });
    }
}
