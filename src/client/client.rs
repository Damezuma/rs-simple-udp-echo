type Client = super::Client;
use std;
impl Client{
    pub fn new()->Result<Client, impl std::error::Error>{
        match std::net::UdpSocket::bind(std::net::SocketAddrV4::new(std::net::Ipv4Addr::from(0), 0)){
            Ok(socket)=>Ok(Client{socket:socket}),
            Err(e)=>Err(e)
        }
    }
    pub fn send(&self, text:&str, to: &std::net::SocketAddrV4){
        self.socket.send_to(text.as_bytes(), to);
    }
}