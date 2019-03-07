use std::io;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::process;
use std::str::from_utf8;
use std::thread;

fn server_handle_client(mut stream: TcpStream) {
        let mut data = [0 as u8; 50]; // using 50 byte buffer
        while match stream.read(&mut data) {
                Ok(size) => {
                        // echo everything!
                        let message = from_utf8(&data).unwrap().to_string();
                        println!("{}", message);
                        println!("{}", message.chars().count());
                        let v: Vec<&str> = message.split(' ').collect();
                        println!("{}", v[2]);
                        stream.write(&data[0..size]).unwrap();
                        true
                }
                Err(_) => {
                        println!(
                                "An error occurred, terminating connection with {}",
                                stream.peer_addr().unwrap()
                        );
                        stream.shutdown(Shutdown::Both).unwrap();
                        false
                }
        } {}
}

pub fn run_server() {
        let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
        // accept connections and process them, spawning a new thread for each one
        println!("Server listening on port 3333");
        for stream in listener.incoming() {
                match stream {
                        Ok(stream) => {
                                println!("New connection: {}", stream.peer_addr().unwrap());
                                thread::spawn(move || {
                                        // connection succeeded
                                        server_handle_client(stream)
                                });
                        }
                        Err(e) => {
                                println!("Error: {}", e);
                                /* connection failed */
                        }
                }
        }
        // close the socket server
        drop(listener);
}

pub fn run_client() {
        match TcpStream::connect("localhost:3333") {
                Ok(mut stream) => {
                        println!("Successfully connected to server in port 3333");
                        let mut message = String::new();

                        message.push_str(&get_input_from_user(String::from(
                                "Enter the first number",
                        )));
                        message.push(' ');
                        message.push_str(&get_input_from_user(String::from(
                                "Enter the other number",
                        )));
                        message.push(' ');
                        message.push(get_operation_from_user());
                        println!("{}", message.chars().count());
                        let msg = message.as_bytes();
                        stream.write(msg).unwrap();
                        println!("Sent Hello, awaiting reply...");
                        let mut data = [0 as u8; 6]; // using 6 byte buffer
                        match stream.read_exact(&mut data) {
                                Ok(_) => {
                                        if &data == msg {
                                                println!("Reply is ok!");
                                        } else {
                                                let text = from_utf8(&data).unwrap();
                                                println!("Unexpected reply: {}", text);
                                        }
                                }
                                Err(e) => {
                                        println!("Failed to receive data: {}", e);
                                        process::exit(0x0100);
                                }
                        }
                }
                Err(e) => {
                        println!("Failed to connect: {}", e);
                }
        }
        println!("Terminated.");
}

fn get_input_from_user(name: String) -> String {
        let mut value_input = String::new();
        print!("Enter the {} >> ", name);
        io::stdout().flush().expect("could not flush");
        io::stdin()
                .read_line(&mut value_input)
                .expect("Error reading from STDIN");
        let trimmed_value = value_input.trim();
        match trimmed_value.parse::<usize>() {
                Ok(i) => return i.to_string(),
                Err(..) => {
                        println!("Error {} is not a valid input", trimmed_value);
                        process::exit(0x0100);
                }
        };
}

fn get_operation_from_user() -> char {
        let mut value_input = String::new();
        print!("Enter + or - >> ");
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
