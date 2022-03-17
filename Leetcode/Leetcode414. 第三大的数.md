```
ID: Leetcode414. 第三大的数
Tag: 水题
```



#### 解法一：hashset去重 + 排序

```rust
use std::collections::HashSet;

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut set: HashSet<i32> = nums.into_iter().collect();
        let mut v: Vec<&i32> = set.iter().collect();
        v.sort();
        if v.len() < 3 {
            return *v[v.len() - 1];
        }

        *v[v.len() - 3]
    }
}
```



#### 解法二：函数式编程1

思路同解法一

```rust
impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut v: Vec<i32> = nums;
        v.sort();
        v.dedup(); // 去重
        if v.len() < 3 {
            return v[v.len() - 1];
        }
        v[v.len() - 3]
    }
}
```


