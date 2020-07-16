#[macro_use]
extern crate clap;

mod player;
mod server;

use server::init_server;

fn main() {
    let matches = clap_app!(myapp =>
        (@arg PORT: --port +takes_value "Sets listening port")
    )
    .get_matches();

    let port = matches.value_of("PORT").unwrap_or("19260");
    println!("Value for port: {}", port);

    init_server(port)
}
