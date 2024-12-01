# 分からなかったところ

## functions2.rs
`for in` 文は、Pythonにおける `for in` , C#, jsにおける `foreach` に該当する。これはイテレーターのそれぞれの要素に対して処理することが可能である。 `a..b` は、 「 `a` から `b` の一つ前まえでの要素を順番に産出（ `yield` ）」する。また、 `a..=b` のようにすると、その両端を含む範囲を指定できる。

```rust
fn main() {
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    // `n`は1, 2, ...., 100のそれぞれの値を取ります。
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}
```