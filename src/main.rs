use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.2:8020").unwrap();
    println!("Server listening");

    for stream in listener.incoming()  {
        match stream {
            Ok(mut stream) => {
                thread::spawn(move || {
                    loop {
                        // define buffer
                        let mut buffer = [0;1024];
                        // read the stream and copy into the buffer
                        stream.read(&mut buffer).unwrap();
                        // write into the stream the buffer
                        stream.write(&buffer).unwrap();
                        break;
                    }
                });
            }
            _ => {}
        }
    }
}
