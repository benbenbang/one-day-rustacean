pub fn inspect(s: &str) {
    println!("{}", s.ends_with('s'))
}

pub fn change(s: &mut String) {
    s.push_str("s")
}

pub fn eat(s: String) -> bool {
    s.contains("a") && s.starts_with("b")
}

pub fn bedazzle(s: &mut String) {
    *s = String::from("sparkly")
}
