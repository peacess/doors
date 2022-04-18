use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use mio::{Events, Interest, Poll, Token};
use mio::net::UdpSocket;

use crate::server::config::Config;
use crate::server::FrameHandles;
use crate::server::Shared;

pub struct ChatServer {
    udp_socket: UdpSocket,
    poll: Poll,
    //字段stop_status并不与其它字段或变量有先后关系，这里只需要可见性，但rust本身没有提供"volatile"可见性，所以这里使用 atomic类型
    stop_status: AtomicBool,
    frame_handle: FrameHandles,
}

impl ChatServer {
    const SENDER: Token = Token(0);
    const ECHOER: Token = Token(1);
    pub fn init(config: &Config) -> Result<ChatServer, anyhow::Error> {
        let poll = Poll::new().expect("");
        let shared = Shared::init(config)?;
        //todo init shared
        let addr = format!("{}:{}", config.ip, config.port).parse()?;
        let udp_socket: UdpSocket = UdpSocket::bind(addr)?;
        // udp_socket.set_nonblocking(true);
        let shared = Arc::new(shared);
        Ok(ChatServer {
            udp_socket,
            poll,
            stop_status: AtomicBool::new(true),
            frame_handle: FrameHandles::new(shared),
        })
    }

    pub fn start(&mut self) {
        let mut events = Events::with_capacity(1024);
        let mut buf = [0; 65535];
        *self.stop_status.get_mut() = false;
        let _ = self.poll.registry().register(&mut self.udp_socket, ChatServer::ECHOER, Interest::READABLE).expect("");

        loop {
            //字段stop_status并不与其它字段或变量有先后关系，这里只需要可见性，但rust本身没有提供"volatile"可见性，所以这里使用 atomic类型
            if self.stop_status.load(Ordering::Relaxed) {
                break;
            }
            //todo what returned after timeout
            self.poll.poll(&mut events, Some(Duration::from_secs(1))).expect("");
            for event in events.iter() {
                match event.token() {
                    ChatServer::ECHOER => {
                        //对于udp来说可以在多线程下recv同一个socket，因为udp包会是一个完整的包，要么收到一个包，要么没有，不会收到半个包的情况。
                        //todo 这里的接收是放下独立的线程中更高效，还是在当前线程中直接接收，这个有等待验证。如果使用io_uring就没有这个问题，因为数据已经接收完成了
                        let n = self.udp_socket.recv(&mut buf).expect("");
                        let bytes = buf[..n].to_vec();
                        self.frame_handle.handle(bytes);
                    }
                    not_token @ _ => {
                        log::error!("'{:?}' is not poll event", not_token);
                    }
                }
            }
        }
    }

    pub fn stop(&mut self) {
        self.stop_status.store(true, Ordering::Relaxed);
        if let Err(e) = self.poll.registry().deregister(&mut self.udp_socket) {
            log::error!("{}",e);
        }
    }

    pub fn uninit(&mut self) {}
}