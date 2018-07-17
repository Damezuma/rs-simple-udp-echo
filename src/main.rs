use std::net::{Ipv4Addr,SocketAddrV4 };
mod client;
mod server;
fn execute_server( ip:Option<Ipv4Addr>, port_number:Option<u16>){
    let ip = match ip{
        Some(ip)=>ip,
        None=>Ipv4Addr::new(0, 0, 0, 0)
    };
    let port_number = match port_number{
        Some(v)=>v,
        None=>{
            eprintln!("포트를 지정해 주시기 바랍니다." );
            return;
        }
    };
    let server = server::Server::new(SocketAddrV4::new(ip, port_number)).unwrap();
    server.recv();
}
fn execute_clinet(ip:Option<Ipv4Addr>, port_number:Option<u16>){
    let ip = match ip{
        Some(ip)=>ip,
        None=>{
            eprintln!("서버 IP를 지정해 주시기 바랍니다." );
            return;
        }
    };
    let port_number = match port_number{
        Some(v)=>v,
        None=>{
            eprintln!("포트를 지정해 주시기 바랍니다." );
            return;
        }
    };
    let mut client = client::Client::new().unwrap();
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
    client.send(&s, &SocketAddrV4::new(ip, port_number));
}
fn main() {
    let mut args = std::env::args();
    let mut port_number = None;
    let mut ip:Option<Ipv4Addr> = None;
    let mut execution:Option<&Fn(Option<_>, Option<_>) > = None;
    
    while let Some(arg) = args.next(){
        if arg.len() > 2 && &arg.as_bytes()[0..2] == b"-p"{
            port_number = Some( arg.as_str()[2..].parse::<u16>().unwrap());
            continue;
        }
        if arg.len() >2 && &arg.as_bytes()[0..3] == b"-ip"{
            let numbers:Vec<u8> = arg.as_str()[3..].split(".").map(|it| it.parse().unwrap()).collect();
            if numbers.len() == 4{
                ip = Some(Ipv4Addr::new(numbers[0], numbers[1], numbers[2], numbers[3]));
            }
            
            continue;                
        }
        match arg.as_str(){
            "-c"|"--client"=>{
                execution = Some(&execute_clinet);
            },
            "-s"|"--server"=>{      
                execution = Some(&execute_server);
            },
            _=>{

            }
        }
    }
    if let Some(v) = execution{
        v(ip, port_number);
    }
    else{
        println!("서버 혹은 클라이언트를 선택하지 않았습니다" );
    }
}
