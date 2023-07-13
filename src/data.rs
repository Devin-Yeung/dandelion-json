#[derive(Debug, PartialEq)]
pub enum ValueType {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array,
    // TODO: Vec<T>?
    Object, // TODO: inner?
}

#[derive(Debug, PartialEq)]
pub struct Value {
    pub(crate) v_type: ValueType,
}
