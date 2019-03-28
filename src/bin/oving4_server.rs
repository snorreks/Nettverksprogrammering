use std::net;
use std::str::from_utf8;

fn read_message(socket: &net::UdpSocket, mut buffer: &mut [u8]) -> String {
    let (number_of_bytes, src_addr) = socket.recv_from(&mut buffer).expect("no data received");
    println!("{:?}", number_of_bytes);
    println!("{:?}", src_addr);
    let message = from_utf8(&buffer).unwrap().to_string();
    message
}

fn send(socket: &net::UdpSocket, receiver: &str, message: String) -> usize {
    println!("sending data");
    let v: Vec<&str> = message.split('|').collect();
    let result = socket
        .send_to(calculate_operation(v[0], v[1], v[2]).as_bytes(), receiver)
        .expect("failed to send message");
    println!("data sent! {}", calculate_operation(v[0], v[1], v[2]));
    result
}

fn init_host() -> net::UdpSocket {
    println!("initializing host");
    let socket = net::UdpSocket::bind("127.0.0.1:3333").expect("failed to bind host socket");

    socket
}

fn main() {
    let mut buf = [0; 512];
    let socket = init_host();
    loop {
        let message = read_message(&socket, &mut buf);
        send(&socket, "127.0.0.1:3334", message);
    }
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
