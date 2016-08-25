extern crate mio;
extern crate slab;

const SERVER: mio::Token = mio::Token(0);

use std::io::Read;

#[derive(Debug)]
struct Connection {
    socket: mio::tcp::TcpStream,
}

struct MyHandler {
    server:  mio::tcp::TcpListener,
    clients: mio::util::Slab<Connection>,
    state:   std::cell::RefCell<i32>,
}

impl mio::Handler for MyHandler {
    type Timeout = ();
    type Message = ();

    fn ready(&mut self, event_loop: &mut mio::EventLoop<MyHandler>, token: mio::Token, _events : mio::EventSet) {
        println!("The server is ready to accept connection");
        match token {
            SERVER => {
                match self.server.accept() {
                    Ok(Some((socket, _addr))) => {
                        println!("Accepted new socket");

                        let token = self.clients.insert(Connection { socket: socket }).unwrap();
                        println!("New token: {:?}", token);

                        event_loop.register(
                            &self.clients[token].socket,
                            token,
                            mio::EventSet::readable(),
                            mio::PollOpt::edge() | mio::PollOpt::oneshot()
                        ).unwrap();
                    }
                    Ok(None) => {
                        println!("The server socket wasn't actually ready");
                    }
                    Err(e) => {
                        println!("Error while accepting: {:?}", e);
                        event_loop.shutdown();
                    }
                }
            }
            _ => {
                let mut action = String::new();
                self.clients[token].socket.read_to_string(&mut action).unwrap();
                match action.as_ref() {
                    "+" => {
                        let mut state = self.state.borrow_mut();
                        *state += 1;
                        println!("State: {}", *state);
                    }
                    "-" => {
                        let mut state = self.state.borrow_mut();
                        *state -= 1;
                        println!("State: {}", *state);
                    }
                    _ => {
                        println!("Dunno what to do");
                    }
                }
            }
        }
    }
}

fn main() {
    let addr = "127.0.0.1:8888".parse().unwrap();
    let server = mio::tcp::TcpListener::bind(&addr).unwrap();

    let mut event_loop = mio::EventLoop::new().unwrap();

    event_loop.register(
        &server, 
        SERVER, 
        mio::EventSet::readable(), 
        mio::PollOpt::edge(),
    ).unwrap();

    event_loop.run(
        &mut MyHandler {
            server:  server,
            clients: mio::util::Slab::new_starting_at(mio::Token(1), 1024),
            state:   std::cell::RefCell::new(0) 
        }
    ).unwrap();
}
