use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    // 绑定到本地端口 3000
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000...");

    // 获取 tcpstream 字节流，可以进行传送，但是是原始字节
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established!");
        let mut buffer = [0; 1024];

        // 读取客户端发送的字节流
        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();

        
    }
    

}