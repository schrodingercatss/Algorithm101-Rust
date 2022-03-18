fn main() {
    // 方法一:
    let mut arr1: [u32; 6] = [1, 3, 2, 5, 6, 4];
    // 方法二: 数组长度为10000，每个值都用true填充
    let arr2 = [true; 10000];
    
    // sort方法，len方法都定义在切片上，数组和vector均可以使用
    println!("the arr1's len is {}", arr1.len());
    arr1.sort();
    println!("{:?}", arr1);
}