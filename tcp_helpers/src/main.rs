use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:8000";
    let listener = TcpListener::bind(addr)
        .await
        .expect("Couldn't start ECHO server");


    println!("Starting ECHO server in: {}", addr);
    loop {
        let (socket, _) = listener.accept().await.expect("Could not accept new msg");
        handle_tcp_echo(socket).await;
    }
}

async fn handle_tcp_echo(socket: TcpStream) {
    let mut buffer = [0u8; 1024];
    let mut socket = socket;
    match socket.read(&mut buffer).await {
        Ok(n) => {
            println!("Connection received. {} bytes read", n);
            if let Err(e) = socket.write_all(&buffer[0..n]).await {
                eprintln!("Error trying to write response {}", e);
            }
        }
        Err(e) => {
            eprintln!("Error trying to read from request: {}", e);
        }
    };
}
