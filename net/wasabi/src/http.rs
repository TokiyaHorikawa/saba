extern crate alloc;
use alloc::format;
use alloc::string::String;
use crate::alloc::string::ToString;
use alloc::vec::Vec;
use saba_core::error::Error;
use saba_core::http::HttpResponse;
use noli::net::lookup_host;

pub struct HttpClient {}

impl HttpClient {
  pub fn new() -> Self {
    Self {}
  }

  pub fn get(&self, host: String, port: u16, path: String) -> Result<HttpResponse, Error> {
    let ips = match lookup_host(&host) {
      Ok(ips) => ips,
      Err(e) => {
        return Err(Error::NetWork(format!(
          "Failed to find IP address: {:#?}",
          e
        )))
      },
    };

    if ips.len() < 1 {
      return Err(Error::NetWork("Failed to find IP address".to_string()));
    }

    let socket_addr: SocketAddr = (ips[0], port).into();

    let mut stream = match TcpStream::connect(socket_addr) {
      Ok(stream) => stream,
      Err(_) => {
        return Err(Error::NetWork(
          "Failed to connect to TCP stream".to_string(),
        ))
      },
    };

    // リクエストラインの構築
    let mut request = String::from("GET /");
    request.push_str(&path);
    request.push_str(" HTTP/1.1\n");

    // ヘッダーの追加
    request.push_str("Host: ");
    request.push_str(&host);
    request.push_str("\n");
    request.push_str("Accept: text/html\n");
    request.push_str("Connection: close\n");
    request.push_str("\n");

    let _bytes_written  = match stream.write(request.as_bytes()) {
      Ok(bytes) => bytes,
      Err(_) => {
        return Err(Error::NetWork(
          "Failed to send a reqest to TCP stream".to_string(),
        ))
      }
    }

    // レスポンスの受信
    let mut received = Vec::new();
    loop {
      let mut buf = [0u8; 4096];
      let bytes_read = match stream.read(&mut buf) {
        Ok(bytes) => bytes,
        Err(_) => {
          return Err(Error::NetWork(
            "Failed to receive a reqest from TCP stream".to_string(),
          ))
        }
      };
      if bytes_read == 0 {
        break;
      }
      received.extend_from_slice(&buf[..bytes_read]);
    }

    match core::str::from_utf8(&received) {
      Ok(response) => HttpResponse::new(response.to_string()),
      Err(e) => Err(Error::NetWork(format!("Invalid received response: {}", e)))
    }
  }
}
