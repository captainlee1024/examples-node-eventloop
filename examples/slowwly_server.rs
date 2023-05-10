use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread::sleep;
use std::time::Duration;

/// 用于测试的延时服务关闭了, 所以在执行主程序之前请先启动该服务模拟网络延时
/// cargo run --example slowwly_server --quiet
fn main() {
    let addr = "127.0.0.1:9527";
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    // 展示request
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request: {:#?}", http_request);
    println!("=====================start handle");
    let two_second = Duration::from_secs(2);
    sleep(two_second);

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    println!("=====================done");
    stream.write_all(response.as_bytes()).unwrap();
}