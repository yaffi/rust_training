fn main() {
    let a = [1, 2, 3, 4, 5];

    assert_eq!(a[ .. ], [1, 2, 3, 4, 5]);
    assert_eq!(a[ .. 3], [1, 2, 3]);
    assert_eq!(a[ ..= 3], [1, 2, 3, 4]);
    assert_eq!(a[1 .. ], [2, 3, 4, 5]);
    assert_eq!(a[1 .. 3], [2, 3]);
    assert_eq!(a[1 ..= 3], [2, 3, 4]);

    assert_eq!( .. , std::ops::RangeFull);
    assert_eq!( .. 3, std::ops::RangeTo { end: 3 });
    assert_eq!( ..= 3, std::ops::RangeToInclusive { end: 3 });
    assert_eq!(1 .. , std::ops::RangeFrom { start: 1 });
    assert_eq!(1 .. 3, std::ops::Range { start: 1, end: 3 });
    assert_eq!(1 ..= 3, std::ops::RangeInclusive::new(1, 3));
}