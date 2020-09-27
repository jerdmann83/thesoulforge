use std::error::Error;
use std::{thread, time};
use std::net::TcpStream;
use std::io::prelude::*;
// use tokio::net::TcpStream;
// use tokio::prelude::*;
// fn to_redis_wire(s: &str) -> Vec<u8> {
//     vec![1, 2, 3]
// }

pub struct RedisError {
    repr: String,
}
impl RedisError {
    pub fn new(repr: &str) -> Self {
        RedisError{
            repr: String::from(repr),
        }
    }
}

impl From<std::io::Error> for RedisError {
    fn from(_: std::io::Error) -> Self {
        RedisError::new("nope")
    }
}

pub type RedisResult<T> = std::result::Result<T, RedisError>;

pub enum Command {
    Set { key: String, val: String },
}

impl Command {
    pub fn new(s: &str) -> RedisResult<Command> {
        let tokens = s.split_whitespace();
        if tokens.count() == 0 {
            return Err(RedisError::new("empty command"));
        }

        let tokens : Vec<&str> = s.split_whitespace().collect();
        match tokens.as_slice() {
            ["set", key, val] => {
                return Ok(Command::Set{
                    key: key.to_string(),
                    val: val.to_string()})
            },
            [] | [..] => {
                return Err(RedisError::new("unhandled command"));
            }
        }
    }
}

fn to_bulk_wire(s: &str) -> Vec<u8> {
    let mut out = String::new();
    let tokens = s.split_whitespace();

    // TODO: git gud at rust
    // count consumes the tokens iterator
    out.push_str(format!("*{}\r\n", tokens.count()).as_str());

    // recreate this iterator so we can use it again
    let tokens = s.split_whitespace();
    for t in tokens {
        out.push_str(format!("${}\r\n{}\r\n", t.len(), t).as_str());
    }
    out.into_bytes()
}

pub struct RedisClient {
    stream: Option<TcpStream>,
    prompt: String,
}

pub fn serialize(cmd: &Command) -> Vec<u8> {
    match cmd {
        Command::Set{key, val} => {
            let tmp = format!("set {} {}", key, val);
            return to_bulk_wire(tmp.as_str());
        }
    }
}

impl RedisClient {
    pub fn new(addr: &str) -> Self {
        // let timeout = time::Duration::from_millis(1000);
        let result = TcpStream::connect(addr);
        let cli_stream : Option<TcpStream>;
        let prompt : String;
        match result {
            Ok(stream) => {
                cli_stream = Some(stream);
                prompt = format!("{}>", addr);
            },
            Err(_) => {
                cli_stream = None;
                prompt = String::from("not connected>");
            },
        }
        RedisClient{
            stream: cli_stream,
            prompt: prompt,
        }
    }

    pub fn send_command(&mut self, c: &Command) -> RedisResult<Vec<u8>> {
        match &mut self.stream {
            Some(stream) => {
                let result = stream.write(serialize(&c).as_slice());
                match result {
                    Ok(_) => {
                        let mut buf: Vec<u8> = Vec::new();
                        loop {
                            println!("next!");
                            let bytes = stream.read(&mut buf[..])?;
                            println!("{}", bytes);
                        }
                        return Ok(buf)
                    },
                    Err(_) => {
                        self.on_disconnect();
                        return Err(RedisError::new("send failed"));
                    }
                }

            },
            None => {
                self.on_disconnect();
                return Err(RedisError::new("not connected"));
            }
        }
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
        let cmd = Command::Set{
            key: String::from("foo"),
            val: counter.to_string(),
        };
        match cli.send_command(&cmd) {
            Ok(response) => {
                println!("{}", String::from_utf8(response).unwrap());
            },
            Err(err) => {
                // println!("{}", err);
                println!("error");
            }
        }
        counter += 1;
        thread::sleep(time::Duration::from_millis(500));
    }
}
