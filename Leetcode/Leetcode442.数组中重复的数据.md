```
ID: Leetcode442. 数组中重复的数据
Tag: 水题、原地哈希
```



#### 解法一：原地哈希

将数组元素值当成对应的下标，给对应下标处的元素加 `len`, 最后统计一下元素值大于`2 * len` 的下标，然后加1，则是数组中重复的元素。

```rust
impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let len = nums.len();
        for i in 0..len {
            let index = nums[i] as usize;
            nums[(index - 1) % len] += len as i32;
        }
        nums.into_iter()
            .enumerate()
            .filter(|(_, val)| *val > 2 * len as i32)
            .map(|(idx, _)| idx as i32 + 1)
            .collect()
    }
}
```
