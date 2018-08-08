extern crate socket_example;

use std::net::TcpListener;
use std::net::TcpStream;
use std::io;
use std::io::Read;
use std::u32;
use std::env;

use socket_example::*;

fn tcp_link_one_block(listener: &TcpListener){
    match listener.accept() {
        Ok((_socket, addr)) => println!("new client{:?}", addr),
        Err(e) => println!("couldn't get client {:?}", e)
    }
}

fn tcp_link_incoming(listener: &TcpListener){
//    loop {
        for mut stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let addr = stream.peer_addr().unwrap();
                    println!("new client {}", addr);

                    let mut s = String::from("");
                    //there is block
                    stream.set_nonblocking(true).expect("can't set no blocking");
                    stream.read_to_string(&mut s);

                    println!("read string is {}", s);


//                    let mut buf = vec![];
//                    let readsize = stream.read(&mut buf);
//                    println!("list is {:?}", buf);
//                    match readsize {
//                        Ok(rusize) =>                 println!("read size is {}", rusize),
//                        Err(e) => println!("err {:?}", e),
//                    }
                },
                Err(e) => {println!("couldn't get client {:?}", e)}
            }
        }
//    }
}

fn tcp_link_incoming_with_noblock(listener: &TcpListener) {
    listener.set_nonblocking(true).expect("can't set no blocking");

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                //read from
                let buf:Vec<u8> = vec![];

            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
//                wait_for_fd();
                continue;
            }
            Err(e) => panic!("encountered IO error: {}", e),
        }
    }
}

fn main(){
//    let listener = TcpListener::bind("127.0.0.1:12306").unwrap();

//    tcp_link_one_block(&listener);
//    tcp_link_incoming(&listener);

//    println!("test");
//    return;
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if(args[1] == "client"){
        let nums = args[2].parse().unwrap();
        socket_example::client::client(nums);
    }else {
        if(args.len() == 3 ){
            if(args[2] == "mio"){
                println!("mio  version");
                server_mio::test_server_mio();
            }else{
//                resource_server::resource_server();
            }
        }else{
            println!("multi threads version");
            socket_example::multi_thread_server::multi_thread_server();
        }
    }

}
