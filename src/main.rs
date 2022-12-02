use std::io::{self, BufReader};
use std::net::{IpAddr, TcpListener, TcpStream};

use ultimate_ttt::game::PlayerType;
use ultimate_ttt::game_loop;

fn wait_for_opponent(port: u16) -> io::Result<(TcpStream, PlayerType)> {
    println!("waiting for opponent to connect");
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;
    let (client, _addr) = listener.accept()?;
    Ok((client, PlayerType::X))
}

fn connect_to_opponent(addr: &str) -> io::Result<(TcpStream, PlayerType)> {
    println!("connecting to opponent");
    let client = TcpStream::connect(addr)?;
    Ok((client, PlayerType::O))
}

fn exit_usage_message(prog_name: &str) -> ! {
    println!("Usage for host: {} -l [port]", prog_name);
    println!("Usage for client: {} -c [addr] [port]", prog_name);
    std::process::exit(-1);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let prog_name = &args[0];
    if args.len() < 3 {
        exit_usage_message(prog_name);
    }

    let should_listen = match args[1].as_str() {
        "-l" => true,
        "-c" => false,
        _ => exit_usage_message(prog_name),
    };

    let result = if should_listen {
        match args[2].parse::<u16>() {
            Ok(port_number) => wait_for_opponent(port_number),
            Err(_) => exit_usage_message(prog_name),
        }
    } else {
        if args[2].parse::<IpAddr>().is_ok() == false {
            exit_usage_message(prog_name);
        }

        if args[3].parse::<u16>().is_ok() == false {
            exit_usage_message(prog_name);
        }

        connect_to_opponent(&format!("{}:{}", &args[2], &args[3]))
    };

    let (stream, player) = match result {
        Ok(tuple) => tuple,
        Err(e) => {
            println!("An error occured while connecting/listening");
            println!("{}", e);
            std::process::exit(-1);
        }
    };

    let read_half = match stream.try_clone() {
        Ok(stream) => BufReader::new(stream),
        Err(_) => {
            println!("An error occured while cloning the socket");
            std::process::exit(-1);
        }
    };

    let write_half = stream;

    match game_loop::start_game_loop(write_half, read_half, player) {
        Ok(()) => (),
        Err(_) => {
            println!("connection error, game aborted");
        }
    }
}
