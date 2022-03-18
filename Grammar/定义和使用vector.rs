fn main() {
    // 方法一：使用宏
    let mut v1 = vec![2, 3, 5, 6];
    // 方法二：使用Vec::new()，编译器会自动推导类型
    let mut v2 = Vec::new();
    v2.push("step");
    // 方法三：使用迭代器构建，但此时必须写明类型
    let mut v3: Vec<i32> = (0..5).collect();
    // 方法四：Vec::with_capacity，提前分配容量
    let mut v4 = Vec::with_capacity(2);
    // 翻转方法
    v3.reverse(); // 定义在切片上，数组和vector均可用

    // 插入方法, 在索引3处插入元素35
    v1.insert(3, 35);

    // 删除方法， 删除索引1处的元素
    v1.remove(1); 
    
    // pop方法，删除并返回最后一个元素，如果为空则返回None
    v1.pop();
}