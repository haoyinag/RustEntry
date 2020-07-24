fn main() {
    //数学运算--下面运算结论：浮点数只能跟浮点数运算，整数只能跟整数运算
    let mut sum = 5 + 10; // 加
    let difference = 95.5 - 4.3; // 减
    let product = sum * 4 * 30; // 乘
    let quotient = difference / 56.7 / 32.2; // 除
    let remainder = 43 % 5 - sum; // 求余
    println!("{}", sum);
    println!("{}", difference);
    println!("{}", product);
    println!("{}", quotient);
    println!("{}", remainder);

    // 可变数据
    sum = product + 10;
    println!("可变的sum {}", sum);

    // 布尔型 字符型
    let flag = false;
    let str = "哈哈哈";
    println!("{}", flag);
    println!("{}", str);

    // 复合类型 i32：表示32位架构整点数  f64：表示64位浮点数 u8：表示8位架构整点数(很少用)
    let param: (i32, f64, u8) = (100, 20.1, 44);
    let (x, y, z) = param;
    println!("{}", x);
    println!("{}", y);
    println!("{}", z);
}
