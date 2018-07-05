mod client;
mod server;
fn main() {
    let mut args = std::env::args();
    let mut port_number = None;
    while let Some(arg) = args.next(){
        match arg.as_str(){
            "-p"|"--port"=>{
                if let Some(v)= args.next(){
                    port_number = Some(v.parse::<u16>().unwrap());
                }
            },
            "-i"|"--ip"=>{

            }
            "-c"|"--client"=>{

            },
            "-s"|"--server"=>{

            },
            _=>{

            }
        }
    }
    let port_number = port_number.unwrap_or(7040);
    let ipv = std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(0, 0, 0, 0), port_number);
    let udp_server = if let Ok(v) = std::net::UdpSocket::bind(ipv){
        println!("Echo server ready to connect in port nunber {}", port_number);
        v
    }
    else{
        println!("Bind Failed in port number {}", port_number);
        return;
    };
    let mut buffer = [0u8; 2048];
    while let Ok((paket_size, ipv)) = udp_server.recv_from(&mut buffer){
        println!("{} received from {}", String::from_utf8_lossy(&buffer[0..paket_size]), ipv);
        udp_server.send_to(&buffer[0..paket_size], ipv);
    }
}
