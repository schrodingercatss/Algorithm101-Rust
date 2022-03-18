```
ID: Leetcode495. 提莫攻击
Tag: 水题
```



#### 解法一：直接遍历

```rust
impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
      // expired: 表示中毒状态解除的时间
        let (mut ans, mut expired) = (0i32, time_series[0]);
        for time in time_series {
          // 如果当前并没有中毒，则直接增加duration即可
            if time >= expired {
                ans += duration;
          // 如果处于中毒状态，则应该只增加前一次中毒状态结束到当前中毒状态结束之间的时间
            } else {
                ans += time + duration - expired;
            }
          // 更新中毒状态结束的时间
            expired = time + duration;
        }
        ans
    }
}
```



#### 解法二：函数式编程1

如果两次攻击之间的间隔`delta`小于`duration`，那么上一次攻击的作用时间就为`delta`；否则上次攻击的作用时间为`duration`，最后加上 `duration`处理最后一次的中毒持续时间。

```rust
impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        (0..time_series.len() - 1)
            .map(|i| duration.min(time_series[i + 1] - time_series[i]))
            .sum::<i32>()
            + duration
    }
}
```



### 解法三：函数式编程2

不懂这个语法，存个档再说，思路同解法二（手动滑稽

```rust
impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        time_series
            .windows(2)
            .fold(0, |acc, arr| acc + duration.min(arr[1] - arr[0]))
            + duration
    }
}
```

