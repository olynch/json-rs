use std::collections::hash_map::HashMap;

pub type JSONArray = Vec<JSONVal>;

pub type JSONObject = HashMap<String, JSONVal>;

pub enum JSONVal {
    Array(JSONArray),
    Object(JSONObject),
    Int(i64),
    Float(f64),
    StringVal(String),
    Bool(bool)
}
