// Importing requirements
use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
// use std::sync::{Arc, Mutex};
use hello::ThreadPool;
use std::thread;
use std::time::Duration;

fn main() {
    // creating a TCP listener
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(move || {
            handle_connection(stream);
        });
    }
}

/// Receives the http request and prints
/// it.
fn handle_connection(mut stream: TcpStream) {
    // Receives the TcpStream and prints
    // the http request it holds
    let buf_reader = BufReader::new(&mut stream);

    // Get first line of request
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    let contents =
        fs::read_to_string(filename).unwrap_or_else(|_| panic!("Cannot find {}", &filename));
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream
        .write_all(response.as_bytes())
        .expect("Unable to send response");
}
