pub mod jsondata;

#[cfg(test)]
mod tests {
    use super::jsondata::*;
    use std::collections::hash_map::HashMap;

    #[test]
    fn name_test() {
        let mut json = HashMap::new();
        json.insert("name".to_string(), JSONVal::StringVal("Owen Lynch".to_string()));
        match json.get(&"name".to_string()) {
            Some(&JSONVal::StringVal(ref ret_val)) => assert_eq!(*ret_val, "Owen Lynch".to_string()),
            Some(_) => panic!("ret_val not a string"),
            None => panic!("Nothing found")
        }
    }

    #[test]
    #[should_fail]
    fn name_test_fail() {
        let mut json = HashMap::new();
        json.insert("name".to_string(), JSONVal::Int(43));
        match json.get(&"name".to_string()) {
            Some(&JSONVal::StringVal(ref ret_val)) => assert_eq!(*ret_val, "Owen Lynch".to_string()),
            Some(_) => panic!("ret_val not a string"),
            None => panic!("Nothing found")
        }
    }
}
