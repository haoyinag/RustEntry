fn main() {
    // let mut a = String::from("333");
    let a = "333"; // 会自动定义类型，相当于 String::from("333");
                   // a = 123; // 会报错，因为上面已经声明为String
    println!("a is {}", a);

    let b = 456;
    println!("b is {0}", b);

    let c = "呵呵哈哈";
    println!("c is {0}", c)
}
