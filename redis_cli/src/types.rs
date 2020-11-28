#[allow(dead_code)]
#[derive(Debug)]
pub struct RedisError {
    repr: String,
}
impl RedisError {
    pub fn new(repr: &str) -> Self {
        RedisError {
            repr: String::from(repr),
        }
    }
}

impl From<std::io::Error> for RedisError {
    fn from(_: std::io::Error) -> Self {
        RedisError::new("io_error")
    }
}

impl From<std::num::ParseIntError> for RedisError {
    fn from(_: std::num::ParseIntError) -> Self {
        RedisError::new("parse_error")
    }
}

impl From<std::char::ParseCharError> for RedisError {
    fn from(_: std::char::ParseCharError) -> Self {
        RedisError::new("str_from_utf8_error")
    }
}

impl From<std::string::FromUtf8Error> for RedisError {
    fn from(_: std::string::FromUtf8Error) -> Self {
        RedisError::new("string_from_utf8_error")
    }
}

impl From<std::str::Utf8Error> for RedisError {
    fn from(_: std::str::Utf8Error) -> Self {
        RedisError::new("str_from_utf8_error")
    }
}

pub type RedisResult<T> = std::result::Result<T, RedisError>;

pub enum Command {
    Set { key: String, val: String },
    Get { key: String },
}

impl Command {
    pub fn new(s: &str) -> RedisResult<Command> {
        let tokens = s.split_whitespace();
        if tokens.count() == 0 {
            return Err(RedisError::new("empty command"));
        }

        let tokens: Vec<&str> = s.split_whitespace().collect();
        match tokens.as_slice() {
            ["set", key, val] => {
                return Ok(Command::Set {
                    key: key.to_string(),
                    val: val.to_string(),
                });
            }
            ["get", key] => {
                return Ok(Command::Get {
                    key: key.to_string(),
                });
            }
            [] | [..] => {
                return Err(RedisError::new("unhandled command"));
            }
        }
    }
}
