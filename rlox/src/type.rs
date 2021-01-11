#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Object,
    Nil,
    Boolean,
    Number,
    String,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    // Object,
    // Nil,
    Boolean(bool),
    Number(f64),
    String(String),
};

}
