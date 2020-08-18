fn main() {
    // let mut a = String::from("333");
    // Rust 是强类型语言，但具有自动判断变量类型的能力。这很容易让人与弱类型语言产生混淆。
    let a = "123"; // 会自动定义类型，相当于 String::from("333");
                   // a = 123; // 会报错，因为上面已经声明为String
    println!("a is {}, a again is {}", a, a);

    let b = 456;
    println!("a is {}, b is {}", a, b);

    let c = "呵呵哈哈";
    println!("a is {}, b is {} ,c is {}", a, b, c)
}
