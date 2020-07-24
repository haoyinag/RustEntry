fn main() {
    let x = 111;
    let y = 22.222;
    println(x, y);
    sanyuan();
}

fn println(x: i32, y: f64) {
    // 整点数只能和整点数比较
    // 浮点数只能跟浮点数比较
    // 整点数/浮点数 不能单独用作布尔表达式 if x {} 这样会报错
    // 和js不一样的地方是条件表达式不用括号()包括
    if x > 100 {
        println!("{}", x);
    } else {
        println!("{}", y);
    }
    if y > 100.00 {
        println!("{}", x);
    } else {
        println!("{}", y);
    }
    // if x {
    //     println!("{}", x);
    // }
}

fn sanyuan() {
    let a = 111;
    // 条件语句，类似js的三元表达式，但js的更好用
    let flag = if a > 100 { true } else { false };
    let num = if a > 100 { 1 } else { -1 };
    println!("{}", flag);
    println!("{}", num);
}
