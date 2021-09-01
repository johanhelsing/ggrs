use crate::network::udp_msg::UdpMessage;
use std::net::{SocketAddr, ToSocketAddrs};

mod udp_socket;

pub(crate) use udp_socket::UdpNonBlockingSocket;

pub trait NonBlockingSocket {
    fn send_to<A: ToSocketAddrs>(&self, msg: &UdpMessage, addr: A);
    fn receive_all_messages(&mut self) -> Vec<(SocketAddr, UdpMessage)>;
}
