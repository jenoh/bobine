use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::str;
use sha1::{Sha1, Digest};
use base64::encode;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8020").unwrap();
    println!("Server listening on 127.0.0.1:8020");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Failed to accept a connection: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer);
    if request.starts_with("GET") {
        let key = extract_sec_websocket_key(&request);
        if let Some(key) = key {
            let response_key = generate_sec_websocket_accept_key(&key);
            let response = format!(
                "HTTP/1.1 101 Switching Protocols\r\n\
                 Connection: Upgrade\r\n\
                 Upgrade: websocket\r\n\
                 Sec-WebSocket-Accept: {}\r\n\r\n",
                response_key
            );
            stream.write_all(response.as_bytes()).unwrap();

            loop {
                let mut buffer = [0; 512];
                let bytes_read = stream.read(&mut buffer).unwrap();
                if bytes_read == 0 {
                    break;
                }

                let message = decode_websocket_frame(&buffer[0..bytes_read]);
                println!("Received message: {}", message);

                let response_frame = encode_websocket_frame(&message);
                stream.write_all(&response_frame).unwrap();
            }
        }
    }
}

fn extract_sec_websocket_key(request: &str) -> Option<String> {
    for line in request.lines() {
        if line.starts_with("Sec-WebSocket-Key:") {
            return Some(line[19..].trim().to_string());
        }
    }
    None
}

fn generate_sec_websocket_accept_key(key: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(key.as_bytes());
    hasher.update(b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
    let result = hasher.finalize();
    encode(result)
}

fn decode_websocket_frame(frame: &[u8]) -> String {
    let payload_len = frame[1] & 0x7F;
    let mask = &frame[2..6];
    let payload = &frame[6..(6 + payload_len as usize)];

    let mut decoded = Vec::with_capacity(payload.len());
    for (i, byte) in payload.iter().enumerate() {
        decoded.push(byte ^ mask[i % 4]);
    }

    String::from_utf8(decoded).unwrap()
}

fn encode_websocket_frame(message: &str) -> Vec<u8> {
    let mut frame = Vec::new();
    frame.push(0b10000001); // Text frame
    frame.push(message.len() as u8);
    frame.extend_from_slice(message.as_bytes());
    frame
}