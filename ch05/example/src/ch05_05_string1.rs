fn main() {
    let mut s1 = "ラズベリー".to_string();
    let mut s2 = String::from("ブラックベリー");

    let s3 = "ストロベリー".to_owned();

    s1.push_str("タルト");

    s2.push('と');

    // s2.push_str(s3);

    s2.push_str(&s3);
}