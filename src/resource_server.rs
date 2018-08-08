use std::net::TcpListener;
use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use std::thread;
use std::io::Read;
use std::time::Duration;

pub fn resource_server() {
//    let mut addrResourceValue = HashMap::new();

//    let addrResource = Arc::new(Mutex::new(addrResourceValue));


    let handle = thread::spawn(move || {
        let listener = TcpListener::bind("0.0.0.0:12306").unwrap();
        println!("server addr is {}", listener.local_addr().unwrap());

        for mut stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let addr = stream.peer_addr().unwrap();
                    println!("new client {}", addr);
//                    let mut resource = addrResource.lock().unwrap();
//                    resource.entry(addr).or_insert(&stream);
//                    stream.set_nonblocking(true);
                    loop {
                        thread::sleep(Duration::from_secs(1));
//                        let mut buf = vec![];
//                        println!("read size is {}", stream.read(&mut buf).unwrap());
//                        while stream.read(&mut buf).unwrap() != 0 {
//                            println!("reading string is {:?}", buf);
//                        }
                        let mut s = String::from("");
                        stream.read_to_string(&mut s);
                        println!("reading string is {}", s);
                    }
                }
                Err(e) => {
                    println!("error is {:?}", e);
                }
            }
        }
    });

    handle.join().unwrap();
}