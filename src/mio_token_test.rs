//use mio::*;
//use mio::net::TcpListener;
//use std::collections::HashMap;
//
//const MAX_SOCKETS: usize = 32;
//const LISTENER: Token = Token(1024);
//
//pub fn test (){
//    let mut sockets =  HashMap::new();
//    let mut next_socket_index = 0;
//
//    let poll = Poll::new().unwrap();
//
//    let listener = TcpListener::bind(&"127.0.0.1")
//}