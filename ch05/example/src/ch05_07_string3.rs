fn f1(name: &str) {
    let s = format!("Hello, {}!", name);
    &s
}

fn f1(name: &str) {
    format!("Hello, {}!", name)
}
