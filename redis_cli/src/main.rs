use std::io::prelude::*;
use std::net::TcpStream;
use std::{thread, time};

mod protocol;
mod types;

use protocol::*;
use types::RedisError;
use types::RedisResult;

#[derive(Debug)]
pub struct RedisClient {
    stream: Option<TcpStream>,
    endpoint: String,
    prompt: String,
}

impl RedisClient {
    pub fn new(addr: &str) -> Self {
        // let timeout = time::Duration::from_millis(1000);
        let result = TcpStream::connect(addr);
        let cli_stream: Option<TcpStream>;
        let prompt: String;
        match result {
            Ok(stream) => {
                cli_stream = Some(stream);
                prompt = format!("{}>", addr);
            }
            Err(_) => {
                cli_stream = None;
                prompt = String::from("not connected>");
            }
        }
        RedisClient {
            stream: cli_stream,
            prompt: prompt,
            endpoint: String::from(addr),
        }
    }

    pub fn send_command(&mut self, cmd: &str) -> RedisResult<Vec<u8>> {
        match self.send_command_impl(cmd) {
            Ok(r) => {
                self.on_connect();
                return Ok(r);
            }
            Err(r) => {
                self.on_disconnect();
                return Err(r);
            }
        }
    }

    fn send_command_impl(&mut self, cmd: &str) -> RedisResult<Vec<u8>> {
        match &mut self.stream {
            Some(stream) => {
                let s = WireType::SimpleString(cmd.to_string());
                // stream.write_all(serialize_simple_str(&cmd).as_slice())?;
                stream.write_all(s.serialize().as_slice())?;
                let bufsize = 1024;
                let mut buf: Vec<u8> = Vec::with_capacity(bufsize);
                for _ in 0..bufsize {
                    buf.push(0);
                }
                stream.read(buf.as_mut_slice())?;
                return Ok(buf);
            }
            None => {
                return Err(RedisError::new("not connected"));
            }
        }
    }

    fn on_connect(&mut self) {
        self.prompt = format!("{}>", self.endpoint);
    }

    fn on_disconnect(&mut self) {
        self.prompt = String::from("not connected>");
    }
}

pub fn main() {
    // ridiculous library driver code
    let mut counter: u32 = 0;
    let mut cli = RedisClient::new("127.0.0.1:6379");
    loop {
        let cmd = format!("set foo {}", counter);
        match cli.send_command(&cmd) {
            Ok(response) => {
                println!("{}", String::from_utf8(response).unwrap());
            }
            Err(_err) => {
                // println!("{}", err);
                println!("error");
            }
        }
        counter += 1;
        thread::sleep(time::Duration::from_millis(500));
    }
}
