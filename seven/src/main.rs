fn main() {
    let mut num = 0;
    // while 循环最典型，用法和 if else条件使用一样
    while num < 5 {
        println!("{}", num);
        num += 1;
    }
    println!("EXIT");
    println!("=============");
    for_array();
    println!("=============");
    use_loop();
    println!("=============");
}

fn for_array() {
    let array = [10, 20, 110, 210, 101, 201];
    // for in 遍历，第一个参数是数组的元素，第二个参数是要遍历的数组下标区间
    for item in 0..array.len() {
        println!("array[{}] = {}", item, array[item]);
    }
}

fn use_loop() {
    let jenny_array = ["J", "E", "N", "N", "Y", "."];
    let mut index = 0;
    // loop 循环如果没有使用条件之后并 break 会报错
    // 在println中输出带某些标识符，和js一样可以用 \ 转义
    loop {
        let index_str = jenny_array[index];
        if index_str == "." {
            break;
        }
        println!("\'{}\'", index_str);
        index += 1;
    }
}
