#[derive(Debug, Eq, PartialEq)]
pub enum ValueType {
    Null,
    Bool(bool),
    Number(isize),
    String(String),
    Array,
    // TODO: Vec<T>?
    Object, // TODO: inner?
}

#[derive(Debug, Eq, PartialEq)]
pub struct Value {
    pub(crate) v_type: ValueType,
}
