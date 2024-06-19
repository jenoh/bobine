use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

fn main() {
    // println!("Hello, world!");
    let listener = TcpListener::bind("127.0.0.2:8020").unwrap();
    println!("Server listening");

    for stream in listener.incoming()  {
        match stream {
            Ok(mut stream) => {
                thread::spawn(move || {
                    loop {
                        let mut read = [0;1028];
                        match stream.read(&mut read) {
                            Ok(n) => {
                                if n== 0 {
                                    break;
                                }
                                println!("read");
                                stream.write(&read[0..n]).unwrap();
                            }
                            Err(err) => {
                                panic!("{}", err)
                            }
                        }
                    }
                });
            }
            _ => {}
        }
    }
}
