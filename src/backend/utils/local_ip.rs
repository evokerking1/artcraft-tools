use std::net::UdpSocket;

pub fn get_local_ip() -> String {
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.connect("8.8.8.8:80").unwrap(); 
    socket.local_addr().unwrap().ip().to_string()
}