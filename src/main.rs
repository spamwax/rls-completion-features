use socket2::SockAddr;
use socket2::{Domain, Socket, Type};

fn main() {
    let socket = Socket::new(Domain::unix(), Type::dgram(), None).unwrap();
    let addr = SockAddr::unix("/tmp/rust.socket").unwrap();
    
    socket.bind(&addr).unwrap();

    let s = socket.into_unix_datagram();
}
