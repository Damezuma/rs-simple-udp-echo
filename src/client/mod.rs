use std::net::Ipv4Addr;
use std::net::UdpSocket;
use std::prelude::v1::*;
mod client;
pub struct Client{
    socket:UdpSocket
}