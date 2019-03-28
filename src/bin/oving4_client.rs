use std::env;
use std::io;
use std::io::{BufRead, BufReader, Error, Read, Write};
use std::net;
use std::process;
use std::str::from_utf8;

fn listen(socket: &net::UdpSocket) -> Vec<u8> {
    let mut buf: [u8; 20] = [0; 20];
    let number_of_bytes: usize = 0;
    let mut result: Vec<u8> = Vec::new();
    match socket.recv_from(&mut buf) {
        Ok((number_of_bytes, src_addr)) => {
            println!("received message: {:?}", buf);
            result = Vec::from(&buf[0..number_of_bytes]);
        }
        Err(fail) => println!("failed listening {:?}", fail),
    }

    let display_result = result.clone();
    let result_str = String::from_utf8(display_result).unwrap();
    println!("received message: {:?}", result_str);
    result
}

fn send(socket: &net::UdpSocket, receiver: &str, msg: &Vec<u8>) -> usize {
    println!("sending message: {:?}", msg);
    let result: usize = 0;
    match socket.send_to(&msg, receiver) {
        Ok(number_of_bytes) => println!("{:?}", number_of_bytes),
        Err(fail) => println!("failed sending {:?}", fail),
    }

    result
}

fn init_host() -> net::UdpSocket {
    let socket = net::UdpSocket::bind("127.0.0.1:3334").expect("failed to bind host socket");
    let duration = std::time::Duration::new(1, 0);
    let dur = std::option::Option::Some(duration);
    let _res = socket.set_read_timeout(dur).expect("failed to set timeout");

    socket
}

fn main() {
    let socket: net::UdpSocket = init_host();
    println!("socket: {:?}", socket);
    loop {
        let mut message = String::new();
        message.push_str(&get_input_from_user(String::from("first number")));
        message.push('|');
        message.push_str(&get_input_from_user(String::from("other number")));
        message.push('|');
        message.push(get_operation_from_user());
        message.push('|');
        let msg_bytes = message.into_bytes();

        send(&socket, "127.0.0.1:3333", &msg_bytes);
        listen(&socket);
    }
}

fn get_input_from_user(name: String) -> String {
    let mut value_input = String::new();
    print!("Enter the {} >> ", name);
    io::stdout().flush().expect("could not flush");
    io::stdin()
        .read_line(&mut value_input)
        .expect("Error reading from STDIN");
    let trimmed_value = value_input.trim();
    match trimmed_value.parse::<isize>() {
        Ok(i) => return i.to_string(),
        Err(..) => {
            println!("Error {} is not a valid input", trimmed_value);
            process::exit(0x0100);
        }
    };
}

fn get_operation_from_user() -> char {
    let mut value_input = String::new();
    print!("Enter + or -           >> ");
    io::stdout().flush().expect("could not flush");
    io::stdin()
        .read_line(&mut value_input)
        .expect("Error reading from STDIN");
    let trimmed_value = value_input.trim();
    match trimmed_value.parse::<char>() {
        Ok(i) => return i,
        Err(..) => {
            println!("Error {} is not a valid input", trimmed_value);
            process::exit(0x0100);
        }
    };
}
