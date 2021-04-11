// 引入标准库中的net、io包的相关结构
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, Error};

// 定义处理客户端消息的handle函数
fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    // 实例化一个用于接收消息的buffer
    let mut buf = [0; 2048];
    
    // 将从客户端接收到的信息读入buffer内，并做错误处理
    stream.read(&mut buf)?;

    // 打印读到的消息至server控制台
    println!("{}", String::from_utf8_lossy(&buf[..]));
    // 将读到的消息写回至client，并做错误处理
    stream.write(&buf[..])?;

    // 函数返回空值
    Ok(())
}
// 定义程序主函数
fn main() -> std::io::Result<()>{
    // 实例化一个tcp server结构，监听本地端口，并做错误处理
    let listener = TcpListener::bind("127.0.0.1:12345")?;

    // 循环迭代server接收到的消息
    for stream in listener.incoming() {
        // 对接收消息的stream结构，做模式匹配
        match stream {
            // 如果有值，将stream结构交给hanle_client函数进行处理
            Ok(stream) => {
                handle_client(stream)?;
            }
            // 如果接收到err，打印err信息至控制台
            Err(e) => {
                println!{"error: {}", e};
            }
        }
    }
    // 函数返回为空值
    Ok(())
}