mod client;
mod server;
fn main() {
    let mut args = std::env::args();
    let mut port_number = None;
    let mut ip:Option<Vec<u16>> = None;
    while let Some(arg) = args.next(){
        if arg.len() > 2 && &arg.as_bytes()[0..2] == b"-p"{
            port_number = Some( arg.as_str()[2..].parse::<u16>().unwrap());
            continue;
        }
        if arg.len() >2 && &arg.as_bytes()[0..3] == b"-ip"{
            ip = Some(arg.as_str()[3..].split(".").map(|it| it.parse::<u16>().unwrap()).collect());
            continue;                
        }
        match arg.as_str(){
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
