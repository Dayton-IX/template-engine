use regex::{Regex, Captures};
use std::{fmt, collections::HashMap};

enum Data {
    Number(i32),
    Boolean(bool),
    Text(String)
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Data::Number(x) => write!(f, "{}", x),
            Data::Boolean(x) => write!(f, "{}", x),
            Data::Text(x) => write!(f, "{}", x)
        }
    }
}

fn render(mut template: String, mut data: HashMap<&str, Data>) -> String {
    let print_regex = Regex::new(r"\{\{(.*?)\}\}").unwrap();
    
    template = print_regex.replace_all(&template, |caps: &Captures| {
        let key = caps.get(1).unwrap().as_str().trim();
        data[key].to_string()
    }).to_string();

    return "Hello World".to_string();
}

