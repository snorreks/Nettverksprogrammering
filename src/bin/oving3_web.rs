use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3000").expect("Could not bind");
    println!("Server listening on port 3000");
    let stream = listener.accept().expect("Error waiting for connection").0;
    handle_request(stream);
}

fn handle_request(stream: TcpStream) {
    let mut reader = BufReader::new(stream);
    let mut request = String::new();

    for line in reader.by_ref().lines() {
        let line = line.unwrap();
        if line == "" {
            break;
        }
        request.push_str("<li>");
        request.push_str(&line);
        request.push_str("</li>");
    }
    send_response(reader.into_inner(), request);
}

// This function takes the stream we just got from the
// listener and then reads some data from it.
fn send_response(mut stream: TcpStream, request: String) {
    let mut response = String::from("HTTP/1.0 200 OK\nContent-Type: text/html\n\n<html><body><H1> Hilsen. Du har koblet deg opp til min enkle web-tjener </h1>Header fra klient er:<ul>");
    response.push_str(&request);
    response.push_str("</ul></body></html>");
    println!("{}", response);
    stream
        .write_all(response.as_bytes())
        .expect("Could not send data");
}
