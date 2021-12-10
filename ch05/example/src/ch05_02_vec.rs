fn main() {
    let v1 = vec![false, true, false];
    let v2 = vec![0.0, -1.0, 1.0, 0.5];

    assert_eq!(v2.len(), 4);

    let v3 = !vec[0; 100];
    assert_eq!(v3.len(), 100);

    let v4 = vec![vec!['a', 'b', 'c'], vec!['d']];

    // let v5 = vec![false, 'a']; 

    let mut v6 = vec!['a', 'b', 'c'];
    v6.push('d');
    v6.push('e');
    assert_eq!(v6, vec!['a', 'b', 'c', 'd', 'e']);

    assert_eq!(v6.pop(), Some('e'));
    v6.insert(1, 'f');
    assert_eq!(v6, vec!['a', 'f', 'c', 'd']);

    let mut v7 = vec!['g', 'h'];
    v6.append(&mut v7);
    assert_eq!(v6, vec!['a', 'f', 'c', 'd', 'g', 'h']);
    assert_eq!(v7, []);

    let a8 = ['i', 'j'];
    v6.extend_from_slice(&a8);

    assert_eq!(v6, vec!['a', 'f', 'c', 'd', 'g', 'h', 'i', 'j']);
    assert_eq!(v8, ['i', 'j']);
}