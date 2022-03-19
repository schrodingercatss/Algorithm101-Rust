```
ID: Leetcode41. 缺失的第一个正数
Tag: 水题、原地哈希
```



#### 解法一：替换法

将元素值当做下标，然后替换到对应的下标处，重复该操作，最后遍历找到第一个不满足调剂的值即可。

```rust
impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let l = nums.len() as i32;
        for i in 0..nums.len() {
            let mut idx = nums[i];
            while idx > 0 && idx <= l && nums[(idx - 1) as usize] != idx {
                std::mem::swap(&mut nums[(idx - 1) as usize], &mut idx);
            }
        }
        for i in 0..l {
            if nums[i as usize] != i + 1 {
                return i + 1;
            }
        }
        l + 1
    }
}
```



### 解法二： 哈希表法

```rust
use std::collections::HashSet;
impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let nums = nums.into_iter().collect::<HashSet<_>>();
        for i in 1.. {
            if !nums.contains(&i) {
                return i
            }
        }
        0
    }
}
```

