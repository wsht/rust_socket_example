use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
pub fn client(loops: u32) {
    let mut i = 0;
    while i < loops {
        let thread_id = i;
        thread::spawn(move || {
            let mut stream = TcpStream::connect("0.0.0.0:12306").expect("could'nt bind to address");
            let mut s = String::from("");

            let send_msg = String::from("abc");

            stream.write(send_msg.as_bytes());
            // stream.write(send_msg.as_bytes());
            // stream.write(send_msg.as_bytes());
            // stream.write(send_msg.as_bytes());
            // stream.write(send_msg.as_bytes());
            // stream.write(send_msg.as_bytes());
            // stream.write(send_msg.as_bytes());
            // stream.write(send_msg.as_bytes());
            //            println!("finish {}", i);
            println!("stream write end and begin receive");
            // thread::sleep(Duration::from_millis(1));
            let mut read_buf = [0; 256];
            loop {
                match stream.read(&mut read_buf) {
                    Ok(0) => {
                        continue;
                    }
                    Ok(_) => {
                        let receive_msg = String::from_utf8(read_buf.to_vec()).unwrap();
                        println!("thread {} read from server {}", thread_id, receive_msg);
                        println!("\n");
                        stream.write(receive_msg.as_bytes());
                        thread::sleep(Duration::from_millis(1));
                    }
                    Err(_) => {}
                }
            }
        });

        i += 1;
        //        thread::sleep(Duration::from_secs(1));
    }

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
