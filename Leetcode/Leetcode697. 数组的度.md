```
ID: Leetcode697. 数组的度
Tag: 水题
```



#### 解法一：哈希表

```rust
use std::collections::HashMap;
impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let (mut max, mut ans) = (0, 0); // 数据出现的最大次数，答案
        let mut map = HashMap::new(); //(num, index): nums[i]出现的次数，index表示第一次出现的下标
        for i in 0..nums.len() {
            // 处理第一次插入的数据
            if let None = map.get(&nums[i]){ map.insert(nums[i], (1, i)); }
            // 对插入过的数据进行处理
            if let Some(&(num, index)) = map.get(&nums[i]) {
                // 若出现次数相同则取最小的len
                if num + 1 == max {
                    ans = ans.min((i + 1 - index) as i32);
                }
                if num + 1 > max {
                    max = num + 1;
                    ans = (i + 1 - index) as i32;
                }
                map.insert(nums[i], (num + 1, index));
            }
        }
        ans
    }
}
```

#### 解法二：哈希表

哈希表记录每个元素第一次出现的位置和最后一次出现的位置

```rust
impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut hash_table  = std::collections::HashMap::new();
        let mut max_value = (0, 0);
        for (pos, num) in nums.iter().enumerate() {
            let entry = hash_table.entry(num).or_insert((0, pos, pos));
            *entry = (entry.0 + 1, entry.1, pos);
            if entry.0 > max_value.0 || (entry.0 == max_value.0 && entry.2 - entry.1 + 1 < max_value.1) {
                max_value = (entry.0, entry.2 - entry.1 + 1);
            }
        }
        max_value.1 as i32
    }
}
```

#### 解法三：哈希表 + 函数式编程

```rust
use std::collections::HashMap;

impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut cnt_left_len = HashMap::new();
        let mut degree = 0;

        for i in 0..nums.len() {
            let cll = cnt_left_len.entry(nums[i]).or_insert([0, i, 1]);
            cll[0] += 1;
            cll[2] = i - cll[1] + 1;
            degree = degree.max(cll[0])
        }

        cnt_left_len.values()
                    .filter(|arr| arr[0] == degree)
                    .map(|arr| arr[2])
                    .min()
                    .unwrap() as i32
    }
}
```

