/*
 * Object := { DictStart }
 *         | [ ArrayStart ]
 *         | StringVal
 *         | Number
 *         | Bool
 *         | Null
 *
 *
 * ArrayStart := Object ArrayEntry
 *             |
 *
 * ArrayEntry := , Object ArrayEntry
 *             | 
 * 
 * DictStart := StringVal : Object DictEntry
 *            | 
 *
 * DictEntry := , StringVal : Object DictEntry
 *            | 
 *
 * FIRST(Object) = {'{', '[', StringVal, Number, Bool, Null}
 * FIRST(ArrayStart) = FIRST(Object)
 * FIRST(ArrayEntry) = {','}
 * FIRST(DictStart) = StringVal
 * FIRST(DictEntry) = {','}
 * FOLLOW(Object) = {',', ':', ']', '}'}
 * FOLLOW(ArrayStart) = {']', ','}
 * FOLLOW(ArrayEntry) = {']', ','}
 * FOLLOW(DictStart) = {'}', ','}
 * FOLLOW(DictStart) = {'}', ','}
 *
 * This SHOULD be LL(1)
 */

use jsondata::*;
use lexer::Token;

#[derive(Debug)]
pub enum Object {
    DS(Box<DictStart>),
    AS(Box<ArrayStart>),
    StringVal(String),
    Number(f64),
    Bool(bool),
    Null
}

#[derive(Debug)]
pub enum DictStart {
    Entry(String, Object, Box<DictEntry>),
    Nil
}

#[derive(Debug)]
pub enum DictEntry {
    Entry(String, Object, Box<DictEntry>),
    Nil
}

#[derive(Debug)]
pub enum ArrayStart {
    Entry(Object, Box<ArrayEntry>),
    Nil
}

#[derive(Debug)]
pub enum ArrayEntry {
    Entry(Object, Box<ArrayEntry>),
    Nil
}

#[derive(Debug)]
pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil
}

impl<T> List<T> {
    fn new() -> List<T> {
        List::Nil
    }
    fn first<'a>(&'a self) -> &'a T {
        match self {
            &List::Cons(ref val, _) => val,
            &List::Nil => panic!("list empty")
        }
    }
    fn rest<'a>(&'a self) -> &'a List<T> {
        match self {
            &List::Cons(_, ref rest) => &**rest,
            &List::Nil => panic!("list empty")
        }
    }
    fn prepend(self, elem: T) -> List<T> {
        List::Cons(elem, box self)
    }
}

pub fn parse(tokenvec: &Vec<Token>) -> Object {
    let mut tokenlist = List::Nil;
    for tok in tokenvec.iter().rev() {
        let t: Token = (*tok).clone();
        tokenlist = tokenlist.prepend(t);
    }
    let (obj, _) = parse_object(&tokenlist);
    obj
}

fn parse_object<'a>(tokenlist: &'a List<Token>) -> (Object, &'a List<Token>) {
    match tokenlist.first() {
        &Token::OCB => {
            let (dict_start, remaining) = parse_dict_start(tokenlist.rest());
            assert_eq!(*remaining.first(), Token::CCB);
            (Object::DS(box dict_start), remaining.rest())
        },
        &Token::OB => {
            let (array_start, remaining) = parse_array_start(tokenlist.rest());
            assert_eq!(*remaining.first(), Token::CB);
            (Object::AS(box array_start), remaining.rest())
        },
        &Token::StringVal(ref val) => (Object::StringVal(val.clone()), tokenlist.rest()),
        &Token::Bool(ref val) => (Object::Bool(val.clone()), tokenlist.rest()),
        &Token::Null => (Object::Null, tokenlist.rest()),
        &Token::Number(ref val) => (Object::Number(val.clone()), tokenlist.rest()),
        _ => panic!("parse error on Token {:?}", *tokenlist.first())
    }
}
fn parse_dict_start<'a>(tokenlist: &'a List<Token>) -> (DictStart, &'a List<Token>) {
    match tokenlist.first() {
        &Token::StringVal(ref val) => {
            assert_eq!(*tokenlist.rest().first(), Token::Colon);
            let (obj, remaining) = parse_object(tokenlist.rest().rest());
            let (next_entry, remaining2) = parse_dict_entry(remaining);
            (DictStart::Entry(val.clone(), obj, box next_entry), remaining2)
        },
        &Token::CCB => {
            (DictStart::Nil, tokenlist)
        },
        _ => {
            panic!("parse error");
            (DictStart::Nil, tokenlist)
        }
    }
}
fn parse_dict_entry<'a>(tokenlist: &'a List<Token>) -> (DictEntry, &'a List<Token>) {
    println!("DictEntry: {:?}", tokenlist.first());
    match tokenlist.first() {
        &Token::Comma => {
            let (key, remaining) = (
                    match tokenlist.rest().first() {
                        &Token::StringVal(ref val) => val.clone(),
                        _ => panic!("parse error")
                    },
                    tokenlist.rest().rest()
                );
            assert_eq!(*remaining.first(), Token::Colon);
            let remaining2 = remaining.rest();
            let (obj, remaining3) = parse_object(remaining2);
            let (next_entry, remaining4) = parse_dict_entry(remaining3);
            (DictEntry::Entry(key, obj, box next_entry), remaining4)
        },
        &Token::CCB => {
            (DictEntry::Nil, tokenlist)
        },
        _ => panic!("parse error")
    }
}
fn parse_array_start<'a>(tokenlist: &'a List<Token>) -> (ArrayStart, &'a List<Token>) {
    match tokenlist.first() {
        &Token::OB | &Token::OCB | &Token::StringVal(_) | &Token::Bool(_) | &Token::Number(_) | &Token::Null => {
            let (obj, remaining) = parse_object(tokenlist);
            let (next_entry, remaining2) = parse_array_entry(remaining);
            (ArrayStart::Entry(obj, box next_entry), remaining2)
        },
        &Token::CB => (ArrayStart::Nil, tokenlist),
        _ => panic!("parse error")
    }
}
fn parse_array_entry<'a>(tokenlist: &'a List<Token>) -> (ArrayEntry, &'a List<Token>) {
    match tokenlist.first() {
        &Token::Comma => {
            let (obj, remaining) = parse_object(tokenlist.rest());
            let (next_entry, remaining2) = parse_array_entry(remaining);
            (ArrayEntry::Entry(obj, box next_entry), remaining2)
        }
        &Token::CB => (ArrayEntry::Nil, tokenlist),
        _ => panic!("parse error")
    }
}
