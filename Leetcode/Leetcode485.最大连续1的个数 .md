```
ID: Leetcode485.最大连续1的个数
Tag: 水题
```



#### 解法一：直接遍历

```rust
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let(mut max, mut cnt) = (0i32, 0i32);
        for num in nums {
            if num == 1 {
                cnt += 1;
            } else {
                max = max.max(cnt);
                cnt = 0;
            }
        }
        max.max(cnt)
    }
}
```



#### 解法二：函数式编程1

```rust
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        nums
            .split(|&x| x == 0) // use `0` value to split vector into slices which only contains `1`
            .map(|v| v.iter().count()) // count each `1` slice's length
            .max() // find the max length
            // because split an empty vector will at least return an empty slice
            // which makes sense, so `max()` will always return `Some(T)`.
            // But the `max()` function signature returns `Option<T>`, we will use `unwrap_or(default)` for that.
            .unwrap_or(0) as i32
    }
}
```



### 解法三：函数式编程2

不懂这个语法，存个档再说（手动滑稽

```rust
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold((0, 0), |(ans, cnt), &n| {
                if n == 1 {
                    (ans.max(cnt + 1), cnt + 1)
                } else {
                    (ans, 0)
                }
            })
            .0
    }
}
```

