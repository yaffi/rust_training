fn main() {
    let p1: Box<i32> = Box::new(10);

    let p2 = p1 as *mut i32;
    // -> error[E0605]: non-primitive cast..

    let p3: *mut i32 = unsafe { std::mem::transmute(p1) };

    let f1 = 5.6789e+3_f32; // 5.6789

    // f32型からi32型にキャストする。小数点以下は切り捨てられる
    let i1 = f1 as i32;
    println!("{}", i1); // 5678

    let i2: i32 = unsafe { std::mem::transmute(f1) };
    println!("{}", i2); // 不動終点数を整数として再解釈した値1169258291が表示される
}