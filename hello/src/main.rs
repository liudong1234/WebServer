use std::{fs, thread};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);  

    for stream in listener.incoming().take(1) {
        let stream = stream.unwrap();
        // handle_connection(stream);

        pool.execuse(|| {
            handle_connection(stream)
        });
        println!("Shutting down.");
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
    //     ("HTTP/1.1 200 OK", "hello.html")
    // }
    // else if request_line == "GET /login HTTP/1.1" {
    //     ("HTTP/1.1 200 OK", "login/index.html")
    // }
    // else{
    //     ("HTTP/1.1 404 NOT FOUND", "404.html")
    // };
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => {
            ("HTTP/1.1 200 OK", "hello.html")
        },
        "GET /login HTTP/1.1" => {
            ("HTTP/1.1 200 OK", "login/index.html")
        },
        _ => {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        }
    };

    let contents = fs::read_to_string(filename).expect("打开文件失败");
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
