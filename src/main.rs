use std::io::BufReader;
use std::net::{IpAddr, TcpListener, TcpStream};

use ultimate_ttt::ai_opponent::AiOpponent;
use ultimate_ttt::game::{PlayerType, UltimateBoard};
use ultimate_ttt::game_loop;
use ultimate_ttt::network_opponent::NetworkOpponent;
use ultimate_ttt::Opponent;

fn wait_for_opponent(port: u16) -> (TcpStream, PlayerType) {
    println!("waiting for opponent to connect");
    let listener = match TcpListener::bind(format!("127.0.0.1:{}", port)) {
        Ok(listener) => listener,
        Err(_) => {
            println!("could not bind to port");
            std::process::exit(-1);
        }
    };

    match listener.accept() {
        Ok((stream, _addr)) => (stream, PlayerType::X),
        Err(_) => {
            println!("error with the incoming connection");
            std::process::exit(-1);
        }
    }
}

fn connect_to_opponent(addr: &str) -> (TcpStream, PlayerType) {
    println!("connecting to opponent");
    match TcpStream::connect(addr) {
        Ok(stream) => (stream, PlayerType::O),
        Err(_) => {
            println!("an error occured while connecting");
            std::process::exit(-1);
        }
    }
}

fn opponent_from_stream(stream: TcpStream) -> impl Opponent {
    let read_half = match stream.try_clone() {
        Ok(stream) => BufReader::new(stream),
        Err(_) => {
            println!("An error occured while cloning the socket");
            std::process::exit(-1);
        }
    };

    let write_half = stream;
    NetworkOpponent::new(write_half, read_half)
}

fn exit_usage_message(prog_name: &str) -> ! {
    println!("Usage for ai: {} ai", prog_name);
    println!("Usage for host: {} host [port]", prog_name);
    println!("Usage for client: {} client [addr] [port]", prog_name);
    std::process::exit(-1);
}

fn parse_args() -> (Box<dyn Opponent>, PlayerType) {
    let args: Vec<String> = std::env::args().collect();
    let prog_name = &args[0];

    if args.len() < 2 {
        exit_usage_message(prog_name);
    }

    match args[1].as_str() {
        "host" => match args[2].parse::<u16>() {
            Ok(port_number) => {
                let (stream, player) = wait_for_opponent(port_number);
                (Box::new(opponent_from_stream(stream)), player)
            }
            Err(_) => exit_usage_message(prog_name),
        },
        "client" => {
            if args[2].parse::<IpAddr>().is_ok() == false {
                exit_usage_message(prog_name);
            }

            if args[3].parse::<u16>().is_ok() == false {
                exit_usage_message(prog_name);
            }

            let (stream, player) = connect_to_opponent(&format!("{}:{}", &args[2], &args[3]));
            (Box::new(opponent_from_stream(stream)), player)
        },
        "ai" => {
            let opponent = AiOpponent::new(UltimateBoard::new());
            (Box::new(opponent), PlayerType::X)
        }
        _ => exit_usage_message(prog_name)
    }
}

fn main() {
    let (mut opponent, player) = parse_args();

    match game_loop::start_game_loop(&mut opponent, player) {
        Ok(()) => (),
        Err(_) => {
            println!("connection error, game aborted");
        }
    }
}
