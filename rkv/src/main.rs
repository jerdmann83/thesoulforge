use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io;
use std::io::Write;
use std::io::prelude::*;
use std::str;

#[macro_use] extern crate log;

// use serde::{Serialize, Deserialize}
//
// try tokio reactor thing

struct KeyValueStore {
    map: HashMap<String, String>
}

fn stream_write(stream: &mut TcpStream, msg: &str) {
    stream.write(msg.as_bytes()).expect("write fail!");
}

impl KeyValueStore {
    fn on_stream(&mut self, mut stream: TcpStream) {
        let mut buf = [0; 1024];
        loop {
            stream_write(&mut stream, "rkv> ");

            let num = stream.read(&mut buf).unwrap_or(0);
            if num == 0 {
                break;
            }

            let s = str::from_utf8(&buf[0..num]).unwrap();

            let tokens: Vec<&str> = s.trim().split_whitespace().collect();

            if tokens.len() == 0 {
                stream_write(&mut stream, "zero_tokens\n");
                continue
            }

            if tokens.len() < 2 {
                let msg = format!("err: invalid command '{}'\n", tokens[0]);
                stream_write(&mut stream, &msg);
                continue
            }

            self.on_command(tokens, &mut stream);
        }
    }

    pub fn on_command(&mut self, tokens: Vec<&str>, stream: &mut TcpStream) {
        match tokens[0] {
            "get" => {
                let result = self.get_key(tokens[1]);
                match result {
                    Some(s) => {
                        stream_write(stream, &format!("\"{}\" => \"{}\"\n",
                                                tokens[1], s));
                    },
                    None => {
                        stream_write(stream, &format!("\"{}\" => (nil)\n",
                                                tokens[1]));
                    },
                }
            },
            "set" => {
                if tokens.len() < 3 {
                    stream_write(stream, "err: set command missing arguments\n");
                    return;
                }
                self.set_key(tokens[1], tokens[2]);

                self.get_key(tokens[1]).unwrap_or("(nil)".to_string());
                stream_write(stream, &format!("\"{}\" => \"{}\"\n",
                                                tokens[1], tokens[2]));
            },
            &_ => {
                stream_write(stream, "unhandled command\n");
            },
        }
    }

    pub fn get_key(&mut self, key: &str) -> Option<String> {
        let result = self.map.get(key);
        match result {
            Some(n) => Some(n.clone()),
            None => None,
        }
    }

    pub fn set_key(&mut self, key: &str, val: &str) {
        self.map.insert(key.to_string(), val.to_string());
    }
}

impl KeyValueStore {
    fn new() -> KeyValueStore {
        let kv = KeyValueStore{
            map: HashMap::new()
        };
        kv
    }
}

fn main() {
    env_logger::init();
    info!("startup");

    let mut kv = KeyValueStore::new();
    let listener = TcpListener::bind("127.0.0.1:9999").unwrap();

    // accept connections and process them serially
    // let mut threads = Vec::new();
    for stream in listener.incoming() {
        // log::info!("accept {}", stream);
        // let t = thread::Builder::new().spawn(|| {
                kv.on_stream(stream.expect("derp"));
        // });
    }
    log::info!("before OK");
    // });
}
