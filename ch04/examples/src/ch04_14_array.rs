fn main() {
    let a1 = [fales, true, false]; // [bool; 3]
    let a2 = [0.0, -0.1, 4.3]; // [f64; 3]

    assert_eq!(a1.len(), a2.len());

    let a3 = [0; 100]; // [i32; 100]
    assert_eq!(a3.len(), 100);

    let a4 = [['a', 'b'],['c', 'd']]; // [[char; 2]; 2]

    let size = 100;
    let mut v1 = vec![0; size];
    assert_eq!(v1.len(), 100);

    v1.push(1);
    assert_eq!(v1.len(), 101);
    assert_eq!(v1.pop(), Some(1));
    assert_eq!(v1.len(), 100);

    let array1 = ['H', 'e', 'l', 'l', 'o'];
    assert_eq!(array1[1], 'e');

    let mut array2 = [0, 1, 2];
    array2[1] = 10;
    assert_eq!(array2, [0, 10, 2]);

    let mut index = 0;
    assert_eq!(array2[index], 0);
    index += 1;
    assert_eq!(array2[index], 10);

    let array3 = [0, 1];
    assert_eq!(array3.get(1), Some(&1));
    assert_eq!(array3.get(2), None);

    let array4 = ['a'; 50];

    for ch in array4.iter() {
        println!("{}", *ch);
    }

    let mut array5 = [1; 50];
    for n in array5.iter_mut() {
        *n *= 2;
    }
}
