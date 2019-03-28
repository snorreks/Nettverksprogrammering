use std::io;
use std::io::{BufRead, BufReader, Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process;
use std::str::from_utf8;
use std::thread;

pub fn run_server() {
        let listener = TcpListener::bind("0.0.0.0:3333").expect("Could not bind");
        // accept connections and process them, spawning a new thread for each one
        println!("Server listening on port 3333");
        for stream in listener.incoming() {
                match stream {
                        Ok(stream) => {
                                thread::spawn(move || {
                                        server_handle_client(stream)
                                                .unwrap_or_else(|error| eprintln!("{:?}", error));
                                });
                        }
                        Err(e) => {
                                eprintln!("Error: {}", e);
                        }
                }
        }
        drop(listener);
}

fn server_handle_client(mut stream: TcpStream) -> Result<(), Error> {
        println!("New connection: {}", stream.peer_addr()?);
        let mut buf = [0; 512]; // using 512 byte buffer
        loop {
                let bytes_read = stream.read(&mut buf)?;
                if bytes_read == 0 {
                        return Ok(());
                }
                let message = from_utf8(&buf).unwrap().to_string();
                let v: Vec<&str> = message.split('|').collect();
                stream.write(calculate_operation(v[0], v[1], v[2]).as_bytes())?;
        }
}

pub fn run_client() {
        let mut stream = TcpStream::connect("localhost:3333").expect("Could not connect to server");
        println!("Successfully connected to server in port 3333");
        loop {
                let mut message = String::new();
                let mut buffer: Vec<u8> = Vec::new();
                message.push_str(&get_input_from_user(String::from("first number")));
                message.push('|');
                message.push_str(&get_input_from_user(String::from("other number")));
                message.push('|');
                message.push(get_operation_from_user());
                message.push('|');
                stream.write(message.as_bytes())
                        .expect("Failed to write to server");
                let mut reader = BufReader::new(&stream);
                reader.read_until(b'.', &mut buffer)
                        .expect("Could not read into buffer");
                println!(
                        "{}",
                        from_utf8(&buffer).expect("Could not write buffer as string")
                );
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

fn calculate_operation(input_a: &str, input_b: &str, operation: &str) -> String {
        let a = input_a.parse::<isize>().unwrap();
        let b = input_b.parse::<isize>().unwrap();
        let mut answer = String::from("* The answer is ");
        if operation == "+" {
                answer.push_str(&(a + b).to_string());
        } else {
                answer.push_str(&(a - b).to_string());
        }
        answer.push('.');
        return answer;
}
