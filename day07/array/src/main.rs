fn main() {
    let mut numbers: Vec<i32> = vec!();

    // 向向量添加元素
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);
    println!("-------------------------------");
    println!("向量：{:?}", numbers);
    println!("长度：{}", numbers.len());
    println!("内存容量：{}", numbers.capacity());

    // 访问向量
    if let Some(first) = numbers.get(0) {
        println!("第一个元素是：{}", first);
    }

    //numbers.pop();// 移出最后一个元素

    //numbers.insert(1, 15);// 插入指定索引元素

    numbers.remove(0);//移出索引元素

    numbers.shrink_to_fit();

    println!("-------------------------------");
    println!("向量：{:?}", numbers);
    println!("长度：{}", numbers.len());
    println!("内存容量：{}", numbers.capacity());

    for number in &numbers {
        println!("Number: {}", number);
    }
}