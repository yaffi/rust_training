fn main() {
    let v1: Vec<u8> = vec![3, 4, 5];
    // let v1 = vec![3u8, 4u8, 5u8];

    assert_eq!(Some(&3u8), v1.first());
    // assert_eq!(Some(&3u8), (&v1[..]).first());

    let mut s1 = String::from("Type coercion");
    let s2 = String::from("is actually ease.");

    s1.push(&s2);
    // (&mut s1).push_str(s2.as_str());

    let mut i1 = 0u8;
    i1 = 255;

    let p1 = &&&&[1, 2, 3, 4]; // &&&&[i32; 4]

    // 型強制が&&&&a1 -> &&&a1 -> &&a1 -> &a1の順に推移的に作用する
    let p2: &[i32; 4] = p1;
}