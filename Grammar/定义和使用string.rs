fn main() {
    // 定义字符串字面量，单引号无需转义，双引号需要转义
    let speech = "\"Ouch!\" said the well.\n";
    // 分行符号
    println!("It was a bright, cold day in April, and \
        there were four of us—\
        more or less.");
    // 如果没有分行符号，空白内容也会被打印出来
    println!("In the room the women come and go,
        Singing of Mount Abora");

    // 原始字符串
    let default_win_install_path = r"C:\Program Files\Gorillas";
    let pattern = Regex::new(r"\d+(\.\d+)*");

    // 在原始字符串中无法直接使用双引号，需要在首尾插入任意个#号表示开头和结尾
    println!(r###"
        This raw string started with 'r###"'.
        Therefore it does not end until we reach a quote mark ('"')
        followed immediately by three pound signs ('###'):
    "###);

    // 字节字符串，实际上是u8值的切片
    // 字节字符串不能包含任意 Unicode 字符，只能是 ASCII 和 \xHH 转义序列。
    let method = b"GET";
    assert_eq!(method, &[b'G', b'E', b'T']);

    // 字符串字面量实际上就是&str类型，无法被修改
    let mut s = "hello";
    
    // 创建String，可修改，跟vector的方法差不多，有new，capacity，push，pop，reverse，切片等语法
    // to_string() 方法 把&str转换成String
    let error_message = "too many pets.".to_string();
    // 还可以使用format!宏返回一个String
    let s2 = format!("{} + {} = {}", 2, 5, 7);

    // 字符串拼接方法 concat和join
    let bits = vec!["veni", "vidi", "vici"];
    assert_eq!(bits.concat(), "venividivici");
    assert_eq!(bits.join(", "), "veni, vidi, vici");
}