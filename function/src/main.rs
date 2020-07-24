fn main() {
    println!("Hello, world!");
    tt_tt();
    with_params(1, 2);
}

// 函数名称必须全小写，可以用_等符号相连
fn tt_tt() {
    println!("This is tt_tt function");
}

// Rust 中定义函数如果需要具备参数必须声明参数名称和类型
fn with_params(x: i32, y: i64) {
    let num = x + 1;
    let num2 = y + 1;
    println!("num 的值为 : {}", num);
    println!("num2 的值为 : {}", num2);
}
