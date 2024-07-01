pub mod consts;
pub mod packet;
pub mod peer;

use crate::{packet::packets::disconnect::Disconnect, peer::Peer};
use tokio::{
    net::{TcpListener, TcpStream},
    spawn,
    time::timeout,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(consts::ADDRESS).await.unwrap();
    println!("listening on {}", consts::ADDRESS);

    loop {
        let (socket, _) = match listener.accept().await {
            Ok(socket) => socket,
            _ => continue,
        };

        let _addr = match socket.peer_addr() {
            Ok(addr) => addr,
            _ => continue,
        };

        spawn(async move {
            println!("{} connected", _addr);
            handle_client(socket).await;
            println!("{} disconnected", _addr);
        });
    }
}

async fn handle_client(socket: TcpStream) {
    let mut peer = Peer::new(socket);
    loop {
        let data = match timeout(consts::PEER_TIMEOUT, peer.next()).await {
            Ok(Ok(data)) => data,
            _ => return,
        };

        #[allow(unused)]
        let packet_type = match data.first() {
            Some(type_) => type_,
            _ => return,
        };

        // will print everything including the packet type, every type length and etc
        dbg!(&data);

        let packet = Disconnect::new_literal("Yippie! A packet!");
        let _ = peer.send(packet).await;
    }
}
