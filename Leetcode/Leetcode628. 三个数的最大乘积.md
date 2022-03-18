```
ID: Leetcode628. 三个数的最大乘积
Tag: 水题
```



#### 解法一：遍历

```rust
impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let (mut min1, mut min2) = (i32::MAX, i32::MAX); // 找出最小的两个数
        let (mut max1, mut max2, mut max3) = (i32::MIN, i32::MIN, i32::MIN); // 找出最大的三个数

        for num in nums {
            // 找最小的两个数
            if num < min1 {
                min2 = min1;
                min1 = num;
            } else if num < min2 {
                min2 = num;
            }

            // 找最大的三个数
            if num > max1 {
                max3 = max2;
                max2 = max1;
                max1 = num;
            } else if num > max2 {
                max3 = max2;
                max2 = num;
            } else if num > max3 {
                max3 = num;
            }
        }
        (max1 * max2 * max3).max(max1 * min1 * min2) 
    }
}
```



#### 解法二：排序

```rust
impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut tmp = nums;
        tmp.sort();
        (tmp[len - 1] * tmp[len - 2] * tmp[len - 3]).max(tmp[0] * tmp[1] * tmp[len - 1])
    }
}
```

