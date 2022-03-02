use std::time::SystemTime;

use signal_hook::consts::TERM_SIGNALS;

use chat_server::chat_server::ChatServer;
use chat_server::config::Config;

fn main() {
    log::info!("start chat server!");
    let config = {
        match Config::load("") {
            Ok(c) => c,
            Err(e) => {
                println!("{:?} \n exist chat_server", e); //还没有初始化日志，所以直接输出
                return;
            }
        }
    };
    //todo init log

    let mut server = {
       match ChatServer::init(&config) {
           Ok(t) => t,
           Err(e) => {
               log::error!("{:?} \n exist chat_server", e);
               return;
           }
       }
    };
    server.start();

    //the duration of ctrl-c is le 10s,then exit server
    {
        let mut t = None;
        for sig in TERM_SIGNALS {
            match t {
                None => {//first push ctrl-c
                    t = Some(SystemTime::now());
                }
                Some(pre) => {
                    match SystemTime::now().duration_since(pre) {
                        Ok(d) => {
                            if d.as_secs() < 10 {
                                break;
                            } else {
                                //reset the time
                                t = Some(SystemTime::now());
                            }
                        }
                        Err(e) => {
                            //reset the time
                            t = Some(SystemTime::now());
                            log::error!("{}", e);
                        }
                    }
                }
            }
            log::info!("Received signal {:?}", sig);
        }
    }
    server.stop();
    server.uninit();
    log::info!("exit chat server!");
}
