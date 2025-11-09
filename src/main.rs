use http_body_util::Empty;
use hyper::Request;
use hyper::body::Bytes;
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;
use http_body_util::BodyExt;
use tokio::io::{AsyncWriteExt as _, self};
use std::env;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    //PHASE 1: File Reading
    //control to be able to only pass one argument
    let args: Vec<String> = env::args().collect();
    //check args length
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    //let requests: Vec<hyper::Request> = gon::parser::parse()
    println!("With text:\n{contents}");
    //PHASE 2: Request building
    let url = "http://localhost:3000".parse::<hyper::Uri>()?;
    let host = url.host().expect("uri has no host");
    let port = url.port_u16().unwrap_or(80);
    let address = format!("{}:{}", host, port);
    let authority = url.authority().unwrap().clone();
    let req = Request::builder()
        .uri(url)
        .header(hyper::header::HOST, authority.as_str())
        .body(Empty::<Bytes>::new())?;

    //PHASE 3: Loop
    let stream = TcpStream::connect(address).await?;
    let io = TokioIo::new(stream);
    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;

    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    let mut res = sender.send_request(req).await?;
    println!("Response status: {}", res.status());

    //idk what is this
    while let Some(next) = res.frame().await {
    let frame = next?;
    if let Some(chunk) = frame.data_ref() {
        io::stdout().write_all(chunk).await?;
    }
}
    Ok(())
}
