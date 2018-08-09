use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::thread;
use std::time;
use std::time::Duration;
pub fn client(loops: u32) {
    let mut i = 0;
    while i < loops {
        let thread_id = i;
        thread::spawn(move || {
            let mut stream =
                TcpStream::connect("47.94.110.11:8001").expect("could'nt bind to address");
            let mut s = String::from("");

            let send_msg = String::from("abc");

            stream.write(send_msg.as_bytes());

            // println!("stream write end and begin receive");
            let mut read_buf = [0; 256];
            let start_time = time::SystemTime::now();
            let mut i = 0;
            loop {
                match stream.read(&mut read_buf) {
                    Ok(0) => {
                        break;
                    }
                    Ok(_) => {
                        let receive_msg = String::from_utf8(read_buf.to_vec()).unwrap();
                        // println!("thread {} read from server {}", thread_id, receive_msg);
                        // println!("\n");
                        stream.write(receive_msg.as_bytes());
                        i += 1;
                        // thread::sleep(Duration::from_millis(1));
                        thread::sleep(Duration::from_secs(1));
                    }
                    Err(_) => {break;}
                    e => {break;}
                }
            }
            let end_time = time::SystemTime::now();
            let duration = end_time.duration_since(start_time);
            println!("{},{},{:?}", thread_id, i, duration);
        });

        i += 1;
        //        thread::sleep(Duration::from_secs(1));
        // thread::sleep_ms(1);
    }

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
