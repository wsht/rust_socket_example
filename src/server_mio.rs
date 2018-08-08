use mio::tcp::TcpListener;
use mio::tcp::TcpStream;
use mio::*;
use std::collections::HashMap;
use std::io;
use std::io::Read;
use std::io::Write;
use std::net::SocketAddr;
use std::thread;
use std::time::Duration;

const SERVER: Token = Token(0);
const CLIENT: Token = Token(1);

const MAX_SOCKETS: usize = 5000;
const LiSTENER: Token = Token(10000);

struct TCPServer {
    address: SocketAddr,
}

impl TCPServer {
    pub fn new(port: u32) -> Self {
        let address = format!("0.0.0.0:{}", port).parse::<SocketAddr>().unwrap();
        TCPServer { address }
    }

    fn run(&mut self) {
        let server = TcpListener::bind(&self.address).expect("could not bind to port");
        let poll = Poll::new().unwrap();
        poll.register(&server, SERVER, Ready::readable(), PollOpt::edge())
            .unwrap();

        let mut sock = TcpStream::connect(&self.address).unwrap();
        poll.register(
            &sock,
            CLIENT,
            Ready::writable() | Ready::readable(),
            PollOpt::edge(),
        ).unwrap();

        let mut events = Events::with_capacity(1024);

        loop {
            poll.poll(&mut events, None).unwrap();
            for event in events.iter() {
                match event.token() {
                    SERVER => {
                        let (_stream, remote) = server.accept().unwrap();
                        println!("connect from {}", remote);
                        thread::sleep(Duration::from_secs(1));
                    }
                    CLIENT => {
                        if (event.readiness().is_readable()) {
                            let mut buf = vec![];
                            sock.read(&mut buf);
                            println!("read from sock is {:?}", buf);
                        }
                    }
                    _ => {
                        println!("unreachable!");
                    }
                }
            }
        }
    }

    fn run2(&mut self) {
        let mut sockets = HashMap::new();
        let mut next_socket_index = 0;

        let poll = Poll::new().unwrap();
        let listener = TcpListener::bind(&self.address).expect("could not bind to port");

        poll.register(&listener, LiSTENER, Ready::readable(), PollOpt::edge())
            .unwrap();

        let mut events = Events::with_capacity(1024);
        let mut buf = [0; 256];
        println!("here");
        loop {
            //            println!("here 2");
            poll.poll(&mut events, None).unwrap();
            for event in events.iter() {
                println!("new event");
                match event.token() {
                    LiSTENER => {
                        loop {
                            match listener.accept() {
                                Ok((socket, address)) => {
                                    if next_socket_index == MAX_SOCKETS {
                                        return ();
                                    }
                                    println!("new client {}", address);
                                    let token = Token(next_socket_index);
                                    next_socket_index += 1;

                                    poll.register(
                                        &socket,
                                        token,
                                        Ready::readable(),
                                        PollOpt::edge(),
                                    ).unwrap();
                                    sockets.insert(token, socket);
                                    //                                    thread::sleep(Duration::from_secs(1));
                                }
                                //there would break
                                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                                    println!("io::ErrorKind::WouldBlock");
                                    break;
                                }
                                e => panic!("err ={:?}", e),
                            }
                        }
                    }
                    token => {
                        println!("token is {:?}", token);
                        loop {
                            match sockets.get_mut(&token).unwrap().read(&mut buf) {
                                Ok(0) => {
                                    sockets.remove(&token);
                                    break;
                                }
                                Ok(msgLen) => {
                                    // println!("receive num is {:?}", msgLen);
                                    println!("token {:?} receive msg {}", &token, msgLen);
                                    let mut msg = String::from_utf8(buf.to_vec()).unwrap();
                                    // println!("receive msg is {}", msg);
                                    // msg.push_str("server\n");
                                    let sendNum =
                                        sockets.get_mut(&token).unwrap().write(msg.as_bytes());
                                }
                                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                                    break;
                                }
                                e => panic!("err={:?}", e),
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn test_server_mio() {
    let mut server = TCPServer::new(8001);

    server.run2();
}
