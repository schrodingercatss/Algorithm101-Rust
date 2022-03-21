```
ID: Leetcode274. H 指数
Tag: 水题
```



#### 解法一：排序+模拟

`rust` 没有隐式类型转换搞下表可太烦人了（

```rust
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut citations = citations;
        citations.sort_unstable();
        let mut h = 0;
        let mut i = (citations.len() - 1);
        // 这里带上 i < n 是因为 i = 0时，再减一会发现回绕, 如果不加这个，就需要反复进行类型转换
        while i >= 0 && i < citations.len() && citations[i] > h {
            h += 1;
            i -= 1;
        }
        h
    }
}
```

#### 解法二：函数式编程1

先排序，然后找出满足要求的值，这里，如果 `value >= len - i` ,表示，后面 `len - i`篇文章引用次数均大于等于`value`。

因为要让 `h` 尽量大，所以需要遍历去找到一个临界点。

```rust
impl Solution {
    pub fn h_index(mut citations: Vec<i32>)  -> i32 {
        citations.sort();
        let len = citations.len();
        for x in citations
            .iter()
            .enumerate()
            .map(|(i,value)|{
                if *value as usize >= len - i {
                    return len - i;
                }
                std::usize::MAX
            }){
            if x != std::usize::MAX {
                return x as i32;
            }
        }
        return 0;
    }
}
```



### 解法三：函数式编程2

更加巧妙的写法，思路同上。

```rust
impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort_unstable_by_key(|&t| std::cmp::Reverse(t));
        (1..citations.len() + 1)
            .into_iter()
            .filter(|&f| citations[f - 1] >= f as i32)
            .last()
            .or(Some(0))
            .unwrap() as i32
    }
}
```



