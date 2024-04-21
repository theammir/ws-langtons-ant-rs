mod game;

use clap::Parser;
use websocket::sync::Server;
use websocket::OwnedMessage;

#[derive(Parser)]
struct Cli {
    #[arg(
        short = 'p',
        long = "port",
        default_value = "8080",
        help = "Server port"
    )]
    port: u16,

    #[arg(
        short = 'd',
        long = "delay",
        default_value = "50",
        help = "Delay between server ticks in milliseconds"
    )]
    delay: u32,

    #[arg(help = "Game field width")]
    width: u8,
    #[arg(help = "Game field height")]
    height: u8,
}

fn main() {
    let args = Cli::parse();

    let server = Server::bind(String::from("127.0.0.1:") + &args.port.to_string()).unwrap();

    for request in server.filter_map(Result::ok) {
        // Spawn a new thread for each connection
        let _spawn = std::thread::spawn(move || {
            if !request.protocols().contains(&"rust-websocket".to_string()) {
                request.reject().unwrap();
                return;
            }

            let client = request.use_protocol("rust-websocket").accept().unwrap();

            let ip = client.peer_addr().unwrap();
            println!("Connection from {}", ip);

            let (_receiver, mut sender) = client.split().unwrap();

            let mut game = game::Game::new(args.width, args.height);

            loop {
                game.tick();

                let mut bytes: Vec<u8> = game
                    .state
                    .iter()
                    .flatten()
                    .collect::<Vec<_>>()
                    .chunks(8)
                    .map(|chunk| {
                        chunk
                            .iter()
                            .enumerate()
                            .fold(0, |acc, (i, &x)| acc | (x << i))
                    })
                    .collect();
                bytes.insert(0, game.h);
                bytes.insert(0, game.w);

                let message = OwnedMessage::Binary(bytes);
                sender.send_message(&message).unwrap();

                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        });
    }
}
