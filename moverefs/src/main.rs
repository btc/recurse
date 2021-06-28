use std::collections::HashMap;

struct Index<'a> {
    strings: Vec<String>,
    refs: Vec<&'a str>,
}

impl<'a> Index<'a> {
    
    // TODO communicate to the compiler that strings and refs will be owned by an object that outlives them both.
    
    pub fn generate() -> Self {
        let strings = load_huge_strings();
        let refs = compute_refs(&strings);
        Self {
            strings,
            refs,
        }
        // compiler errors: returning strings, but &strings is borrowed
    }

    pub fn generate_with_early_instantiation() -> Self {
        let mut s = Self {
            strings: load_huge_strings(),
            refs: vec![],
        };
        let refs = compute_refs(&s.strings);
        s.refs = refs;
        return s;
        // compiler errors: returning s but &s.strings is borrowed
    }
}

fn compute_refs(strings: &[String]) -> Vec<&str> {
    let mut out = Vec::new();
    for (i, s) in strings.iter().enumerate() {
        for line in s.lines() {
            if is_of_interest(line) {
                out.push(line);
            }
        }
    }
    return out;
}

fn load_huge_strings() -> Vec<String> {
    // load 3GB of strings
    Default::default()
}

fn is_of_interest(s: &str) -> bool {
    todo!()
}

fn main() {
    println!("Hello, world!");
}
