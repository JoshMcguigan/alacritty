use std::sync::Arc;
use crate::sync::FairMutex;
use crate::term::Term;
use crate::util::thread;
use crate::ansi::Handler;
use std::net::{TcpListener, TcpStream};

pub struct CommLoop {
    terminal: Arc<FairMutex<Term>>,
}

impl CommLoop {
    pub fn new(terminal: Arc<FairMutex<Term>>) -> Self {
        CommLoop { terminal }
    }

    pub fn spawn(self) {
        thread::spawn_named("comm loop", move || {
            let listener = TcpListener::bind("127.0.0.1:33111").unwrap();

            for stream in listener.incoming() {
                let term = self.terminal.clone();
                thread::spawn(move || {
                    handle_client(stream.unwrap(), term);
                });
            }
        });
    }
}

fn handle_client(stream: TcpStream, terminal: Arc<FairMutex<Term>>) {
    trace!("Client connected");
    let mut terminal = terminal.lock();
    terminal.bell();
}
