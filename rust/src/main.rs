use std::time::SystemTime;

use signal_hook::consts::TERM_SIGNALS;

use chat_server::config::Config;
use chat_server::run::ChatServer;

fn main() {
    log::info!("start chat server!");
    let config = Config::load("");
    let mut server = ChatServer {};
    server.init(&config);
    server.start();

    //the duration of ctrl-c is le 10s,then exit server
    {
        let mut t = None;
        for sig in TERM_SIGNALS {
            match t {
                None => {
                    t = Some(SystemTime::now());
                }
                Some(last) => {
                    match SystemTime::now().duration_since(last) {
                        Ok(d) => {
                            if d.as_secs() < 10 {
                                break;
                            } else {
                                //reset the time
                                t = Some(SystemTime::now());
                            }
                        }
                        Err(e) => {
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
