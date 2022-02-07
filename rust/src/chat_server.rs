use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use mio::{Events, Interest, Poll, Token};
use mio::net::UdpSocket;

use crate::config::Config;

pub struct ChatServer {
    udp_socket: UdpSocket,
    poll: Poll,
    stop_status: AtomicBool,
}

impl ChatServer {
    const SENDER: Token = Token(0);
    const ECHOER: Token = Token(1);
    pub fn init(config: &Config) -> ChatServer {
        let poll = Poll::new().expect("");
        let addr = format!("{}:{}", config.ip, config.port).parse().expect("");
        let udp_socket: UdpSocket = UdpSocket::bind(addr).expect("");
        // udp_socket.set_nonblocking(true);
        ChatServer {
            udp_socket,
            poll,
            stop_status: AtomicBool::new(true),
        }
    }

    pub fn start(&mut self) {
        let mut events = Events::with_capacity(1024);
        let mut buf = [0; 65535];
        *self.stop_status.get_mut() = false;
        let _ = self.poll.registry().register(&mut self.udp_socket, ChatServer::ECHOER, Interest::READABLE).expect("");

        loop {
            if self.stop_status.load(Ordering::SeqCst) {
                break;
            }
            //todo what returned after timeout
            self.poll.poll(&mut events, Some(Duration::from_secs(1))).expect("");
            for event in events.iter() {
                match event.token() {
                    ChatServer::ECHOER => {
                        let n = self.udp_socket.recv(&mut buf).expect("");
                    }
                    not_token @ _ => {
                        log::error!("'{:?}' is not poll event", not_token);
                    }
                }
            }
        }
    }

    pub fn stop(&mut self) {
        self.stop_status.store(true, Ordering::SeqCst);
        if let Err(e) = self.poll.registry().deregister(&mut self.udp_socket) {
            log::error!("{}",e);
        }
    }

    pub fn uninit(&mut self) {}
}