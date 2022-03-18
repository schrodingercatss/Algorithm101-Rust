```
ID: Leetcode645. 错误的集合
Tag: 水题
```



#### 解法一：哈希表法

```rust
impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut cnt = vec![0; nums.len() + 1];
        for n in nums {
            cnt[n as usize] += 1;
        }
        let mut ans = vec![0; 2];
        for (i, &c) in cnt.iter().enumerate().skip(1) {
            if c == 2 {
                ans[0] = i as i32;
            } else if c == 0 {
                ans[1] = i as i32;
            }
        }
        ans
    }
}
```



#### 解法二：排序+数学公式法

```rust
impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let sum = ((1 + nums.len()) * nums.len() / 2) as i32; // 正确总和
        let cur_sum = nums.iter().sum::<i32>(); // 原数组总和
        nums.sort();
        nums.dedup(); // 去重
        let distinct_sum = nums.iter().sum::<i32>(); // 去重总和
      	// (正确总和 - 去重总和 = 缺失的元素值)
        // (当前总和 - 去重总和 = 重复元素值)
        vec![cur_sum - distinct_sum, sum - distinct_sum]
    }
}
```

