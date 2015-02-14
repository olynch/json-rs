//! Walks the generated tree to create a hashmap
use parser::{Object, DictStart, DictEntry, ArrayStart, ArrayEntry};
use std::collections::hash_map::HashMap;
use jsondata::JSONVal;

pub fn compile(obj: &Object) -> JSONVal {
    match obj {
        &Object::StringVal(ref val) => JSONVal::StringVal(val.clone()),
        &Object::Number(ref val) => JSONVal::Number(val.clone()),
        &Object::Bool(ref val) => JSONVal::Bool(val.clone()),
        &Object::AS(ref array_start) => JSONVal::Array(compile_array(array_start)),
        &Object::DS(ref dict_start) => JSONVal::Object(compile_dict(dict_start)),
        &Object::Null => JSONVal::Null
    }
}

fn compile_array(array_start: &Box<ArrayStart>) -> Vec<JSONVal> {
    let mut array = vec![];
    let mut next = &ArrayEntry::Nil;
    match array_start {
        &box ArrayStart::Entry(ref obj, ref next_entry) => {
            array.push(compile(obj));
            next = (&**next_entry).clone();
        },
        &box ArrayStart::Nil => {
            return array;
        }
    };
    loop {
        match next {
            &ArrayEntry::Entry(ref obj, ref next_entry) => {
                array.push(compile(obj));
                next = &**next_entry;
            },
            &ArrayEntry::Nil => {
                return array;
            }
        }
    }
}

fn compile_dict(dict_start: &Box<DictStart>) -> HashMap<String, JSONVal> {
    let mut dict: HashMap<String, JSONVal> = HashMap::new();
    let mut next = &DictEntry::Nil;
    match dict_start {
        &box DictStart::Entry(ref key, ref obj, ref next_entry) => {
            dict.insert(key.clone(), compile(obj));
            next = &**next_entry;
        },
        &box DictStart::Nil => {
            return dict;
        }
    };
    loop {
        match next {
            &DictEntry::Entry(ref key, ref obj, ref next_entry) => {
                dict.insert(key.clone(), compile(obj));
                next = &**next_entry;
            },
            &DictEntry::Nil => {
                return dict;
            }
        }
    }
}
