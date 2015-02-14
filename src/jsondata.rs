use std::collections::hash_map::HashMap;

pub enum JSONVal {
    Array(Vec<JSONVal>),
    Object(HashMap<String, JSONVal>),
    Number(f64),
    StringVal(String),
    Bool(bool),
    Null
}
