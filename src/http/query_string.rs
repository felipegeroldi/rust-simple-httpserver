use std::{collections::HashMap};

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>)
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &'buf str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();
        
        for item in s.split('&') {
            let mut key = item;
            let mut value = "";
            
            if let Some(i) = item.find('=') {
                key = &item[..i];
                value = &item[i+1..];
            }

            data.entry(key)
            .and_modify(|existing_value: &mut Value| match existing_value {
                Value::Single(p) => {
                    *existing_value = Value::Multiple(vec![p, value]);
                },
                Value::Multiple(vec) => vec.push(value)
            })
            .or_insert(Value::Single(value));
        }

        QueryString { data }
    }
}