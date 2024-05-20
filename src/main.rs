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

        match socket.set_nodelay(true) {
            Ok(_) => (),
            _ => continue,
        }

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
        let packet_type = match data.get(0) {
            Some(type_) => type_,
            _ => return,
        };
        println!("Received packet type: {}", packet_type);

        let packet = Disconnect::new_literal("Yippie! A packet!");
        let _ = peer.send(packet).await;
    }
}
