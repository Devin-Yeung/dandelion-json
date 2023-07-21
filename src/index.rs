// Code adapted from the `serde_json` crate by `dtolnay`.
// Original `serde_json` crate: https://github.com/dtolnay/serde_json
use crate::data::Value;
use std::collections::HashMap;

pub trait Index {
    /// Return None if the key is not already in the array or object.
    #[doc(hidden)]
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value>;

    /// Return None if the key is not already in the array or object.
    #[doc(hidden)]
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value>;

    /// Panic if array index out of bounds. If key is not already in the object,
    /// insert it with a value of null. Panic if Value is a type that cannot be
    /// indexed into, except if Value is null then it can be treated as an empty
    /// object.
    #[doc(hidden)]
    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value;
}

impl Index for usize {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        match v {
            Value::Array(vec) => vec.get(*self),
            _ => None,
        }
    }
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
        match v {
            Value::Array(vec) => vec.get_mut(*self),
            _ => None,
        }
    }

    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
        match v {
            Value::Array(vec) => {
                let len = vec.len();
                vec.get_mut(*self).unwrap_or_else(|| {
                    panic!(
                        "cannot access index {} of JSON array of length {}",
                        self, len
                    )
                })
            }
            _ => panic!("cannot access index {} of JSON", self),
        }
    }
}

impl Index for str {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        match v {
            Value::Object(map) => map.get(self),
            _ => None,
        }
    }
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
        match v {
            Value::Object(map) => map.get_mut(self),
            _ => None,
        }
    }

    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
        if let Value::Null = v {
            *v = Value::Object(HashMap::new());
        }

        match v {
            Value::Object(map) => map.entry(self.to_owned()).or_insert(Value::Null),
            _ => panic!("cannot access key {:?} in JSON", self),
        }
    }
}

impl Index for String {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        self[..].index_into(v)
    }
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
        self[..].index_into_mut(v)
    }

    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
        self[..].index_or_insert(v)
    }
}

// So that we can use syntax like: json["key"]
impl<'a, T> Index for &'a T
where
    T: Index + ?Sized,
{
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value> {
        // *self -> &T
        // **self -> *(*self) -> *(&T) -> T
        (**self).index_into(v)
    }

    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value> {
        (**self).index_into_mut(v)
    }

    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value {
        (**self).index_or_insert(v)
    }
}

impl<I> std::ops::Index<I> for Value
where
    I: Index,
{
    type Output = Value;

    fn index(&self, index: I) -> &Self::Output {
        static NULL: Value = Value::Null;
        index.index_into(&self).unwrap_or(&NULL)
    }
}

impl<I> std::ops::IndexMut<I> for Value
where
    I: Index,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        index.index_or_insert(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::data::Value;
    use crate::parser::Parser;
    use crate::{nums, str};

    #[test]
    fn immutable_index() {
        let json = r#"
        {
            "n" : null ,
            "f" : false ,
            "t" : true ,
            "i" : 123 ,
            "s" : "abc",
            "a" : [ 1, 2, 3 ],
            "o" : { "1" : 1, "2" : 2, "3" : 3 }
        }
        "#;
        let value = Parser::parse(json).unwrap();
        assert_eq!(value["n"], Value::Null);
        assert_eq!(value["f"], Value::Bool(false));
        assert_eq!(value["t"], Value::Bool(true));
        assert_eq!(value["i"], Value::Number(123.0));
        assert_eq!(value["s"], str!("abc"));
        assert_eq!(value["a"], nums!(1, 2, 3));
        let object = &value["o"];
        assert_eq!(object["1"], Value::Number(1.0));
        assert_eq!(object["2"], Value::Number(2.0));
        assert_eq!(object["3"], Value::Number(3.0));
    }

    #[test]
    fn mutable_index() {
        let mut value = Parser::parse("{}").unwrap();
        value["key"] = Value::Bool(true);
        assert_eq!(value["key"], Value::Bool(true));
    }

    #[test]
    fn out_of_bound_null() {
        let value = Parser::parse("[0, 1, 2]").unwrap();
        assert_eq!(&value[3], &Value::Null);
    }

    #[test]
    #[should_panic]
    fn out_of_bound_panic() {
        let mut value = Parser::parse("[0, 1, 2]").unwrap();
        value[3] = Value::Number(3.0);
    }
}
