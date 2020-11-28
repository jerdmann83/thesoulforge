use crate::types::*;

pub type WireVec = Vec<WireType>;

#[derive(Debug, Clone)]
pub enum WireType {
    SimpleString(String),
    BulkString(String),
    Error(String),
    Integer(i32),
    Array(WireVec),
}

pub trait Wire {
    fn serialize(&self) -> Vec<u8>;
}

impl Wire for WireType {
    fn serialize(&self) -> Vec<u8> {
        let mut out = String::new();
        match self {
            WireType::SimpleString(s) => {
                out.push_str(&format!("+{}\r\n", s));
            }
            WireType::Error(s) => {
                out.push_str(&format!("-{}\r\n", s));
            }
            WireType::Integer(i) => {
                out.push_str(&format!(":{}\r\n", i));
            }
            WireType::BulkString(s) => {
                let tmp = &format!("${}\r\n{}\r\n", &s.len(), &s);
                println!("{}", tmp);
                out.push_str(tmp);
            }
            WireType::Array(v) => {
                out.push_str(&format!("*{}\r\n", v.len()));
                for elem in v.iter() {
                    out.push_str(std::str::from_utf8(&elem.serialize()).unwrap());
                }
            }
        }
        out.into_bytes()
    }
}

impl Wire for WireVec {
    fn serialize(&self) -> Vec<u8> {
        let mut out = String::new();
        out.push_str(&format!("*{}\r\n", self.len()));
        for elem in self.iter() {
            out.push_str(std::str::from_utf8(&elem.serialize()).unwrap());
        }
        out.into_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wire_types() {
        let val = WireType::Integer(99);
        let wire_val = val.serialize();
        assert!(matches!(deserialize(&wire_val), Ok(val)));
        // assert!(deserialize(&wire_val) == Ok(val));

        let val = WireType::Error("ERR: bad stuff".to_string());
        let wire_val = val.serialize();
        assert!(matches!(deserialize(&wire_val), Ok(val)));

        let val = WireType::SimpleString("ERR: bad stuff".to_string());
        let wire_val = val.serialize();
        assert!(matches!(deserialize(&wire_val), Ok(val)));

        let val = WireType::BulkString("foobar foobaz frobniz".to_string());
        let wire_val = val.serialize();
        assert!(matches!(deserialize(&wire_val), Ok(val)));

        let mut arr = WireType::Array(vec![]);
        // val.push(WireType::Integer(1));
        // val.push(WireType::Integer(2));
        // val.push(WireType::SimpleString("Hello".to_string()));
        // val.push(WireType::Error("Goodbye".to_string()));
        // let wire_val = val.serialize();
        // let tmp = wire_val.clone();
        // println!("{}", String::from_utf8(tmp).unwrap());
        // if let Err(e) = deserialize(&wire_val) {
        //     println!("{:?}", e);
        // }
        // assert!(matches!(deserialize(&wire_val), Ok(val)));
    }
}

pub fn deserialize(bytes: &[u8]) -> RedisResult<WireType> {
    match bytes {
        [] => {
            return Err(RedisError::new("empty"));
        }
        [b':', rest @ ..] => {
            let s = std::str::from_utf8(rest)?;
            let end = s.rfind("\r\n").unwrap_or(rest.len());
            let i = s[0..end].parse::<i32>()?;
            return Ok(WireType::Integer(i));
        }
        [b'-', rest @ ..] => {
            let s = String::from_utf8(rest.to_vec())?;
            let end = s.rfind("\r\n").unwrap_or(rest.len());
            return Ok(WireType::Error(s[0..end].to_string()));
        }
        [b'+', rest @ ..] => {
            let s = String::from_utf8(rest.to_vec())?;
            let end = s.rfind("\r\n").unwrap_or(rest.len());
            return Ok(WireType::SimpleString(s[0..end].to_string()));
        }
        [b'$', rest @ ..] => {
            let s = format!(
                "${}\r\n{}",
                rest.len(),
                String::from_utf8(rest.to_vec()).unwrap()
            );
            return Ok(WireType::BulkString(s));
        }
        [b'*', rest @ ..] => {
            let rest = std::str::from_utf8(rest)?;
            let mut num_elements: u32;
            let mut element_size: u32;
            // simple finite state automata
            // there has to be a simpler / more idiomatic way to do this?
            enum ParseState {
                Init,
                Size,
                Payload,
                Done,
            };
            let mut state = ParseState::Init;
            loop {
                println!("rest: '{}'", rest);
                match state {
                    ParseState::Init => {
                        let (head, rest) = rest.split_at(1);
                        num_elements = head.parse()?;
                        state = ParseState::Size;
                    }
                    ParseState::Size => {
                        let (head, rest) = rest.split_at(1);
                        let q = head.parse::<char>()?;
                        if q != '?' {
                            return Err(RedisError::new("missing size prefix"));
                        }
                        state = ParseState::Payload;
                    }
                    ParseState::Payload => {
                        element_size = rest.parse()?;
                        state = ParseState::Size;
                    }
                    ParseState::Done => break,
                }
            }
            return Err(RedisError::new("invalid"));
        }

        _ => {
            return Err(RedisError::new("unhandled"));
        }
    }
}
