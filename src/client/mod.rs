use std::net::Ipv4Addr;
use std::net::UdpSocket;
use std::prelude::v1::*;
use std;

mod client;
pub struct Client{
    socket:UdpSocket
}
impl Client{
    pub fn new()->Result<Client, impl std::error::Error>{
        match std::net::UdpSocket::bind(std::net::SocketAddrV4::new(std::net::Ipv4Addr::from(0), 0)){
            Ok(socket)=>Ok(Client{socket:socket}),
            Err(e)=>Err(e)
        }
    }
    pub fn send(&mut self, text:&str, to: &std::net::SocketAddrV4)->Result<String, std::io::Error>{
        self.socket.set_read_timeout(Some(std::time::Duration::from_millis(10)));
        loop{
            let mut buffer = [0u8; 2048];
            if let Err(e) = self.socket.send_to(text.as_bytes(), to){
                return Err(e);
            }
            match self.socket.recv_from(&mut buffer){
                Ok((s, a))=>{
                    let res = String::from_utf8_lossy(&buffer[0..s]).to_string();
                    return Ok(res);
                },
                Err(e)=>{
                    if let std::io::ErrorKind::TimedOut = e.kind(){
                        continue;
                    }
                    return Err(e);
                }
            }
        }
    }
}