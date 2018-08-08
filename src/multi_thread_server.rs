use std::net::TcpListener;
use std::io::Read;
use std::thread;


//server 堵塞监听， 每有一个链接，加入一个线程进行处理监听
pub fn multi_thread_server(){

    let listener = TcpListener::bind("127.0.0.1:12306").unwrap();
    println!("server addr is {}", listener.local_addr().unwrap());
    for mut stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let addr = stream.peer_addr().unwrap();
                println!("new client {}", addr);

                //thread handle accept
                thread::spawn(move|| {
                    let mut s = String::from("");

                    //there blocking
                    stream.read_to_string(&mut s);
                    println!("read string is {}", s);
                });
            }
            Err(e) => {println!("couldn't get client {:?}", e);}
        }
    }
}
