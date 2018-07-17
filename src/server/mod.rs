use std::net::Ipv4Addr;
use std::net::UdpSocket;
use std::prelude::v1::*;
use std;
mod server;
pub struct Server{
    socket:UdpSocket
}
impl Server{
    pub fn new(ipv:std::net::SocketAddrV4)->Result<Server, impl std::error::Error>{
        match std::net::UdpSocket::bind(ipv){
            Ok(socket)=>Ok(Server{socket:socket}),
            Err(e)=>Err(e)
        }
    }
    pub fn recv(&self)->Result<(String, std::net::SocketAddr), impl std::error::Error>{
        let mut buffer =[0u8; 4096];
        match self.socket.recv_from(&mut buffer){
            Ok((size, ipv))=>{
                self.socket.send_to(&buffer[0..size], ipv);
                return Ok((String::from_utf8_lossy(&buffer[0..size]).to_string(), ipv));
            }
            Err(e)=>return Err(e)
        };
    }
}