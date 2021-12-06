fn double(n: i32) -> i32 {
    n + n
}

fn abs(n:i32) -> i32 {
    if n >= 0 { n } else { -n }
}

fn main() {
    let mut t1 = (88, true);

    assert_eq!(t1.0, 88);

    assert_eq!(t1.1, true);

    t1.0 += 100;

    assert_eq!(t1.0, 188);

    let (n1, b1) = (88, true);

    assert_eq!(n1, 88);
    assert_eq!(b1, true);

    let ((x1, y1), (x2, y2)) = ((0, 5), (10, -1));
    // let ((x1, y1), _) = ((0, 5), (10, -1));
    assert_eq!(x1, 0);
    assert_eq!(y1, 5);
    assert_eq!(x2, 10);
    assert_eq!(y2, -1);

    let mut t1 = ((0, 5), (10, -1));
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t1;

    *x1_ptr += 3;
    *y1_ptr *= -1;

    assert_eq!(t1, ((3, -5), (10, -1));)
}