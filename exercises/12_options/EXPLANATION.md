# 分からなかったところ

## `options1.rs`

### Option

プログラムの一部が期待しない値を受け取った時、 `panic` するよりも、エラーを補足したほうが望ましいケースがある。これは `Option` という列挙型を用いることで可能になる。 `Option<T>` は `Some(T)` と `None` の２値を持つ列挙型で、値が存在する場合としない場合を表現する。

```Rust
fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None // 割り算が定義されない場合
    } else {
        Some(a / b) // 値が存在する場合
    }
}

fn main() {
    let result = divide(10, 2);
    match result {
        Some(value) => println!("Result: {}", value), // 値を使用
        None => println!("Division by zero"), // 値がない場合
    }
}
```

また `unwrap` は値を取り出す方法で、 `Option` 型が `Some` であれば値を取り出し、そうでなければ `panic` する。

```Rust
let value = Some(42);
println!("{}", value.unwrap()); // 42
```

```Rust
let value: Option<i32> = None;
println!("{}", value.unwrap()); // パニック
```


## `options2.rs`

### `if let` 

`if let` は、値が特定のパターンに一致する場合にコードブロックを実行する。構文は以下の通り。

```Rust
if let パターン = 値 {
    // パターンに一致した場合に実行されるコード
}
```

Optionの場合は

```Rust
fn main() {
    let some_value = Some(42);

    // some_value が Some であれば値 v を取り出して表示
    if let Some(v) = some_value {
        println!("The value is: {}", v);
    }
    // パターンが一致しない場合 
    else {
        println!("No value found.");
    }
}
```

Resultの場合は

```Rust
fn main() {
    let result: Result<i32, &str> = Ok(100);

    if let Ok(value) = result {
        println!("Success: {}", value);
    } else {
        println!("An error occurred.");
    }
}
```

### `while let` 

`let while` も同様で、パターンに一致する限り、コードブロックを実行し続ける。

```Rust
while let パターン = 値 {
    // パターンに一致する間、繰り返し実行されるコード
}
```
問題では、以下の通りになる。

```Rust
def main(){
    let range = 10;
    let mut optional_integers: Vec<Option<i8>> = vec![None];

    for i in 1..=range {
        optional_integers.push(Some(i));
    }

    let mut cursor = range;

    // TODO: Make this a while-let statement. Remember that `Vec::pop()`
    // adds another layer of `Option`. You can do nested pattern matching
    // in if-let and while-let statements.
    while let Some(Some(integer)) = optional_integers.pop() {
        assert_eq!(integer, cursor);
        cursor -= 1;
    }
}
```

`Some` がネストされている理由は、 `optional_integers.pop()` の返り値がネストされた Option 型（ `Option<Option<i8>>` ）であるためである。 `Option<Option<i8>>` が 存在し、かつ `Option<i8>` が存在している場合のみ、 while ループを続行する。

## `options3.rs`

optionとsemanticsの問題

```Rust
fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    // TODO: Fix the compiler error by adding something to this match statement.
    match optional_point {
        Some(p) => println!("Co-ordinates are {},{}", p.x, p.y),
        _ => panic!("No match!"),
    }

    println!("{optional_point:?}"); // Don't change this line.
}
```

- `match` 文の `Some(p)` に届いた時点で、 `optional_point` は所有権を消費してしまうので、 `println!("{optional_point:?}");` でコンパイルエラーとなってしまう。
- 借用を使う。