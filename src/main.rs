use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    println!("start");
    let listener = TcpListener::bind("127.0.0.1:2022").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream)
    }
}

// fn handle_connection(mut stream: TcpStream) {
//     println!("------stream start--------");
//     let mut buffer = [0; 1024];
//     stream.read(&mut buffer).unwrap();
//     println!("{}", String::from_utf8_lossy(&buffer[..]));
//     let response = "HTTP/1.1 200 OK \r\n\r\n";
//     stream.write(response.as_bytes()).unwrap();
//     stream.flush().unwrap();
//     println!("------stream end--------");
// }

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    print!("Request: {:#?}", http_request);

    // let response = "HTTP/1.1 200 OK \r\n\r\n ad=asdfas \r\n\r\n";
    // stream.write(response.as_bytes()).unwrap();
    // stream.flush().unwrap();
    // println!("----- steam end ------")

    // path is relative to cargo.toml
    let content = fs::read_to_string("./hello.html").unwrap();
    let status_line = "HTTP/1.1 200 OK";
    let length = content.len();

    let response = format!(
        "{}\r\nContent-Length:{length}\r\n\r\n{content}",
        status_line
    );
    stream.write_all(response.as_bytes()).unwrap();
}

