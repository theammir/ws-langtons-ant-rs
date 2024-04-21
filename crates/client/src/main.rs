use websocket::ClientBuilder;
use websocket::OwnedMessage;

use clap::{ArgAction, Parser};

// •✖
#[derive(Parser)]
struct Cli {
    #[arg(
        short = 'p',
        long = "port",
        default_value = "8080",
        help = "Server port"
    )]
    port: u16,

    #[arg(short = 'D', long = "debug", help = "Enable debug mode", action = ArgAction::SetTrue)]
    debug: bool,
}

fn main() {
    let args = Cli::parse();

    let client = ClientBuilder::new(&(String::from("ws://127.0.0.1:") + &args.port.to_string()))
        .unwrap()
        .add_protocol("rust-websocket")
        .connect_insecure()
        .unwrap();

    println!("Connected to server");

    let (mut receiver, _sender) = client.split().unwrap();

    for message in receiver.incoming_messages() {
        if let OwnedMessage::Binary(mut data) = message.unwrap() {
            // Clear the screen
            print!("\x1B[2J\x1B[H");
            if args.debug {
                println!("{:?}", data);
            }

            let width = data.remove(0);
            let height = data.remove(0);
            let size = width as usize * height as usize;

            let bytes = data
                .iter()
                .flat_map(|byte| {
                    (0..8)
                        .map(|i| (byte & (1 << i) != 0) as u8)
                        .collect::<Vec<u8>>()
                })
                .collect::<Vec<u8>>();
            for row in bytes[0..size].chunks(width as usize) {
                for &cell in row {
                    print!("{}", if cell == 1 { "✖  " } else { "•  " });
                }
                println!();
            }
        }
    }
}
