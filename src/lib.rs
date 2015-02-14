#![feature(box_syntax)]
#![feature(box_patterns)]
#[allow(unstable)]

pub mod jsondata;
use jsondata::JSONVal;
mod lexer;
mod parser;
mod compiler;

fn parse_json(json: String) -> JSONVal {
    match lexer::lex(json) {
        Ok(ref tokenvec) => compiler::compile(&parser::parse(tokenvec)),
        Err(_) => panic!("parse error")
    }
}

#[cfg(test)]
mod tests {
    use super::parse_json;
    use super::jsondata;
    use super::jsondata::JSONVal;
    use super::lexer;
    use super::parser;
    use super::compiler;
    use super::lexer::Token::*;
    use std::collections::hash_map::HashMap;

    #[test]
    fn name_test() {
        let mut json = HashMap::new();
        json.insert("name".to_string(), jsondata::JSONVal::StringVal("Owen Lynch".to_string()));
        match json.get(&"name".to_string()) {
            Some(&jsondata::JSONVal::StringVal(ref ret_val)) => assert_eq!(*ret_val, "Owen Lynch".to_string()),
            Some(_) => panic!("ret_val not a string"),
            None => panic!("Nothing found")
        }
    }

    #[test]
    #[should_fail]
    fn name_test_fail() {
        let mut json = HashMap::new();
        json.insert("name".to_string(), jsondata::JSONVal::Number(43 as f64));
        match json.get(&"name".to_string()) {
            Some(&jsondata::JSONVal::StringVal(ref ret_val)) => assert_eq!(*ret_val, "Owen Lynch".to_string()),
            Some(_) => panic!("ret_val not a string"),
            None => panic!("Nothing found")
        }
    }

    #[test]
    fn lexer_test1() {
        let lexed_cmp = vec![
            OCB,
            StringVal("name".to_string()),
            Colon,
            StringVal("Owen Lynch".to_string()),
            Comma,
            StringVal("info".to_string()),
            Colon,
            OCB,
            StringVal("age".to_string()),
            Colon,
            Number(16 as f64),
            CCB,
            CCB
        ];
        let test_json =
        "
        {
            \"name\": \"Owen Lynch\",
            \"info\": {
                \"age\": 16
            }
        }
        ".to_string();
        match lexer::lex(test_json) {
            Ok(lexed) =>
            {
                for (a, b) in lexed_cmp.iter().zip(lexed.iter()) {
                    assert_eq!(a, b);
                }
            },
            Err(_) => panic!("Failed to lex"),
        }
    }

    #[test]
    fn parser_test1() {
        let test_json = 
        "
        {
            \"name\": \"Owen Lynch\",
            \"info\": {
                \"age\": 16
            }
        }
        ".to_string();
        let tokenvec = match lexer::lex(test_json) { Ok(v) => v, Err(_) => panic!("Failed to lex") };
        let parsed = parser::parse(&tokenvec);
        let parsed_cmp = parser::Object::DS(
            box parser::DictStart::Entry(
                "name".to_string(),
                parser::Object::StringVal("Owen Lynch".to_string()),
                box parser::DictEntry::Entry(
                        "info".to_string(),
                        parser::Object::DS(
                                box parser::DictStart::Entry(
                                        "age".to_string(),
                                        parser::Object::Number(16 as f64),
                                        box parser::DictEntry::Nil)),
                        box parser::DictEntry::Nil)));
        let mut json_map = HashMap::new();
        json_map.insert("name".to_string(), JSONVal::StringVal("Owen Lynch".to_string()));
        let mut info_map = HashMap::new();
        info_map.insert("age".to_string(), JSONVal::Number(16 as f64));
        json_map.insert("info".to_string(), JSONVal::Object(info_map));
        let json = JSONVal::Object(json_map);
        let json_parsed = compiler::compile(&parsed);
        let json_parsed_cmp = compiler::compile(&parsed_cmp);
        match (json, json_parsed, json_parsed_cmp) {
            (JSONVal::Object(dict), JSONVal::Object(dict_parsed), JSONVal::Object(dict_parsed_cmp)) => {
                let key = "name".to_string();
                match (dict.get(&key), dict_parsed.get(&key), dict_parsed_cmp.get(&key)) {
                    (Some(&JSONVal::StringVal(ref a)), Some(&JSONVal::StringVal(ref b)), Some(&JSONVal::StringVal(ref c))) => {
                        assert_eq!(*a, *b);
                        assert_eq!(*b, *c);
                    },
                    (_, _, _) => panic!("not able to compare")
                };
                let key2 = "info".to_string();
                match (dict.get(&key2), dict_parsed.get(&key2), dict_parsed_cmp.get(&key2)) {
                    (Some(&JSONVal::Object(ref info_a)), Some(&JSONVal::Object(ref info_b)), Some(&JSONVal::Object(ref info_c))) => {
                        let key = "age".to_string();
                        match (info_a.get(&key), info_b.get(&key), info_c.get(&key)) {
                            (Some(&JSONVal::Number(ref a)), Some(&JSONVal::Number(ref b)), Some(&JSONVal::Number(ref c))) => {
                                assert_eq!(*a, *b);
                                assert_eq!(*b, *c);
                            },
                            (_, _, _) => panic!("not able to compare")
                        };
                    }
                    (_, _, _) => panic!("not able to compare")
                };
            },
            (_, _, _) => panic!("not able to compare")
        }
    }
    #[test]
    fn json_parser_test() {
        let test_json = 
        "
        {
            \"name\": \"Owen Lynch\",
            \"info\": {
                \"age\": 16
            }
        }
        ".to_string();
        parse_json(test_json);
    }
}
