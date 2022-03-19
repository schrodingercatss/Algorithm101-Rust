```
ID: Leetcode448. 找到所有数组中消失的数字
Tag: 水题
```



#### 解法一：哈希表 + 遍历

```rust
impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut ret: Vec<i32> = (0..nums.len()).map(|x| x as i32 + 1).collect();
        nums.iter().for_each(|i| { ret[*i as usize - 1] = 0; });
        ret.into_iter().filter(|x| { *x != 0 }).collect()
    }
}
```



#### 解法二： 原地哈希

把数组中的每一个元素都当成下标，然后将对应下标的元素变成负数，最后遍历数组，找到非负数的下表，即为未出现的数。

```rust
impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        if nums.len() == 0 { return vec![] }
        let mut i = 0;
        loop {
            let index = (nums[i]).abs() as usize -1;
            nums[index] = - nums[index].abs();
            i += if i+1 == nums.len() { break; } else { 1 };
        }
        nums.iter().enumerate().filter(|(_, x)| **x > 0 ).map(|(i, _)| i as i32 +1).collect()
    }
}
```



