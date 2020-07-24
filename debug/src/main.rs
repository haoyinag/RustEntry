fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    // Rust 不支持 ++ 和 --
    let mut a = 1;
    a += 1;
    // a ++; 报错
    println!("The a is {}", a);

    let mut boolean = false;
    println!("The boolean is {}", boolean);
    boolean = true;
    println!("The boolean is {}", boolean);

    let mut d = [3; 5];
    // 等同于 let d = [3, 3, 3, 3, 3];
    d[3] = 444 + 1;
    // 等同于 let d = [3, 3, 3, 444, 3];
    println!("The value of d[3] is: {}", d[3]);

    let sum: (i32, f64, u8) = (1, 2.1, 3);
    let (x, y, z) = sum;
    println!("The x is {}", x);
    println!("The y is {}", y);
    println!("The z is {}", z);
}
