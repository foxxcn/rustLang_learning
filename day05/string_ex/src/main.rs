fn main() {
    let s = String::from("你好, Rust!");

    // 打印原始字符串
    println!("Original string: {}", s);

    // 打印字符串的字节表示
    println!("UTF-8 encoded bytes: {:?}", s.as_bytes());

    // 逐个字符打印及其 Unicode 码点
    for c in s.chars() {
        println!("Character: {}, Unicode: U+{:X}", c, c as u32);
    }

    // 逐个字节打印
    for b in s.bytes() {
        println!("Byte: {:X}", b);
    }
}
